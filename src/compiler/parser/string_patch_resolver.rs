// Used to keep track of changes to source code
// over the course of compilation, allowing all
// outputted code to be mapped back to the original.
// Including over multiple source files

use std::mem;


#[derive(Debug)]
struct Patch{
    start : usize,
    end : usize,
    len_mod : isize
}

#[derive(Debug)]
pub struct PatchString{
   internal_string : String,
   patches : Vec<Patch>,
   is_scalped : bool
}

pub enum RebuildAction{
    Keep,
    DiscardAmount(usize),
    DiscardAndInsert(usize, &'static str),
    DiscardAndInsertString(usize, String),
}



impl PatchString{
    #[inline]
    pub fn new(input : String) -> PatchString{
        PatchString{
            internal_string: input,
            patches: Vec::new(),
            is_scalped: false,
        }
    }
    #[inline]
    pub fn from(input : &str) -> PatchString{
        Self::new(input.to_string())
    }
    #[inline]
    pub fn get_str<'a>(&'a self) -> &'a str{
        assert!(!self.is_scalped);
        &self.internal_string
    }
    // Scalps the PatchString, Gives you the back a string
    // while leaving the PatchString an empty husk
    pub fn scalp(&mut self) -> String{
        self.is_scalped = true;
        mem::take(&mut self.internal_string)
    }
    
    // WARNING: O(N + O)
    // Requires copying both the input string, and copying any remainder
    // in the patch string pos.len() to the right
    pub fn insert(&mut self, pos : usize, str : &str){
        assert!(!self.is_scalped);
        self.internal_string.insert_str(pos, str);
        self.patches.push(Patch{
            start: pos,
            end: pos,
            len_mod: str.len() as isize,
        });
    }
    // WARNING: O(N)
    // Requires copying the remainder of the string end-start to the left
    pub fn delete(&mut self, start : usize, end : usize){
        assert!(!self.is_scalped);
        if start == end{ return };

        let real_end = end.max(start);
        let real_start = end.min(start);
        

        self.internal_string.replace_range(real_start..real_end, "");
        self.patches.push(Patch{
            start: real_start,
            end: real_end,
            len_mod: real_start as isize - real_end as isize,
        });
    }


    pub fn from_mod_index(&self, mod_index: usize) -> usize {
        let mut index = mod_index as isize;
        
        for patch in self.patches.iter().rev() {
            if index < patch.start as isize { }
            else if patch.len_mod > 0 && index < patch.start as isize + patch.len_mod{index = patch.start as isize -1}
            else if index >= patch.start as isize {
                index -= patch.len_mod;
            }
        
        }
        index.max(0) as usize
    }
    pub fn to_mod_index(&self, old_index: usize) -> usize {
        let mut index = old_index as isize;
    
        for patch in &self.patches {

            if index < patch.start as isize { }
            else if index >= patch.end as isize { index += patch.len_mod; } 
            else if patch.len_mod < 0 {
                index = patch.start as isize;
                
            } 
            
        }

        index.max(0) as usize
    }
    #[inline]
    pub fn construct_from<const N : usize>(input : &str, mut predicate : impl FnMut([char; N]) -> RebuildAction) -> Self{
        Self::_construct_from(input, Vec::new(), predicate)
    }
    fn _construct_from<const N : usize>(input : &str, mut patches : Vec<Patch>, mut predicate : impl FnMut([char; N]) -> RebuildAction) -> Self{
        assert!(N != 0, "A window cannot be zero!");

        let mut window : [char; N] = ['\x00'; N];
        let mut chrs = input.char_indices();

        let mut new_string = String::with_capacity(input.len());

        // This is the index in the MODIFIED STRING, not the old string
        let mut start_of_window_index = 0;



        macro_rules! return_mid_window{
            ($into_window:expr) => {
                for i in 0..$into_window{
                    new_string.push(window[i]);
                };
                return Self{
                    internal_string : new_string,
                    patches
                };
            };
        }
        macro_rules! rebuild_window {
            () => {
                for i in 0..N{
                    let char_opt = chrs.next();
                    window[i] = if let Some((_, char)) = char_opt {
                        char
                    } else{
                        return_mid_window!(i);
                    }; 
                }
            };
        }
        macro_rules! discard {
            ($amount : expr) => {{
                let mut amount = $amount;
                patches.push(Patch{
                    start: start_of_window_index,
                    end: amount + start_of_window_index,
                    len_mod: -(amount as isize),
                });
                
                if amount > N{
                    amount -= N;
                    for _ in 0..amount{chrs.next();}

                    rebuild_window!();
                }else{
                    // Equivalent to: window[0..N-amount] = window[amount..N];
                    unsafe{ 
                        std::ptr::copy(window.as_ptr().add(amount), window.as_mut_ptr(), N-amount)
                    }

                    for i in N-amount..N{
                        window[i] = if let Some((_, char)) = chrs.next() {char} else{
                            return_mid_window!(i);
                        }; 
                    }
                }
            }};
        }
  
        macro_rules! advance {
            ($do_err:literal) => {{
                start_of_window_index += 1;
                    
                if N != 1{
                    // Equivalent to: window[0..N-1] = window[1..N];
                    unsafe{ std::ptr::copy(window.as_ptr().add(1), window.as_mut_ptr(), N-1) };
                }
                window[N-1] = if let Some((_, char)) = chrs.next(){
                    char
                }else{
                    // If we are in a discard we do not want to keep any
                    // remaining window chars, as such such use 0. 
                    // Which does nothing but return
                    return_mid_window!(if $do_err {N-1} else {0});
                };
            }};
        }
        macro_rules! keep {
            () => {
                    // Move widow over one
                    new_string.push(window[0]);
                    advance!(true);
            };
        }
        rebuild_window!();
        loop{
            let ack = predicate(window.clone());
            match ack {
                RebuildAction::Keep | RebuildAction::DiscardAmount(0) => {
                    keep!();
                }
                RebuildAction::DiscardAmount(amount) =>{
                    discard!(amount);
                },
                RebuildAction::DiscardAndInsert(_, _) | RebuildAction::DiscardAndInsertString(_, _) =>{
                    let (amount, str) = match &ack {
                        RebuildAction::DiscardAndInsert(n, str) => (*n, *str),
                        RebuildAction::DiscardAndInsertString(n, string) => (*n, string.as_str()),
                        _ => unreachable!()
                    };
                    new_string.push_str(&str);
                    patches.push(Patch { start: start_of_window_index, end: start_of_window_index, len_mod: str.len() as isize });
                    
                    
                    start_of_window_index += str.len();
                    if amount > 1{
                        discard!(amount);
                    }else if amount == 1{
                        advance!(false);
                    }else{
                        // Amount is zero
                        keep!();
                    }
                    
                }
                _ => todo!()
         
            } 
        }
    }
    #[inline]
    pub fn rebuild_string_windowed<const N : usize>(&mut self, predicate : impl FnMut([char; N]) -> RebuildAction){
        assert!(!self.is_scalped);

        // Steal patches, this way we don't need to copy every patch
        let patches = std::mem::take(&mut self.patches);
        *self = Self::_construct_from(self.internal_string.as_str(), patches, predicate);
   
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_mod_index_no_patches() {
        let patch_string = PatchString::new(String::from("hello world"));
        assert_eq!(patch_string.to_mod_index(0), 0);
        assert_eq!(patch_string.to_mod_index(5), 5);
        assert_eq!(patch_string.to_mod_index(11), 11);
    }
    #[test]
    fn test_to_mod_index_with_insertions() {
        let mut patch_string = PatchString::new(String::from("hello world"));
        patch_string.insert(5, " dear"); // Inserts " dear" at position 5

        // Indexes before the patch should stay the same
        assert_eq!(patch_string.to_mod_index(0), 0);
        assert_eq!(patch_string.to_mod_index(4), 4);

        // Indexes at and after the patch should be adjusted by the length of the insertion
        assert_eq!(patch_string.to_mod_index(5), 10); // Position 5 should become 10
        assert_eq!(patch_string.to_mod_index(6), 11);
        assert_eq!(patch_string.to_mod_index(11), 16);
    }
    #[test]
    fn test_to_mod_index_with_deletions() {
        let mut patch_string = PatchString::new(String::from("hello world"));
        patch_string.delete(5, 11); // Deletes " world" (from index 5 to 11)

        // Indexes before the patch should stay the same
        assert_eq!(patch_string.to_mod_index(0), 0);
        assert_eq!(patch_string.to_mod_index(4), 4);

        // Indexes in the deleted range should map to the start of the patch
        assert_eq!(patch_string.to_mod_index(5), 5);
        assert_eq!(patch_string.to_mod_index(6), 5);
        assert_eq!(patch_string.to_mod_index(10), 5);

        // Indexes after the deleted range should be adjusted by the deletion length
        assert_eq!(patch_string.to_mod_index(11), 5);
    }
    #[test]
    fn test_from_mod_index_no_patches() {
        let patch_string = PatchString::new(String::from("hello world"));
        assert_eq!(patch_string.from_mod_index(0), 0);
        assert_eq!(patch_string.from_mod_index(5), 5);
        assert_eq!(patch_string.from_mod_index(11), 11);
    }
    #[test]
    fn test_from_mod_index_with_insertions() {
        let mut patch_string = PatchString::new(String::from("hello world"));
        patch_string.insert(5, " dear"); // Inserts " dear" at position 5

        // Indexes before the patch should stay the same
        assert_eq!(patch_string.from_mod_index(0), 0);
        assert_eq!(patch_string.from_mod_index(4), 4);

        // Indexes after the patch should map back by subtracting the length of the insertion
        assert_eq!(patch_string.from_mod_index(10), 5); // Position 10 should map to original position 5
        assert_eq!(patch_string.from_mod_index(11), 6);
        assert_eq!(patch_string.from_mod_index(16), 11);
    }
    #[test]
    fn test_from_mod_index_with_deletions() {
        let mut patch_string = PatchString::new(String::from("hello world"));
        patch_string.delete(5, 11); // Deletes " world" (from index 5 to 11)

        // Indexes before the patch should stay the same
        assert_eq!(patch_string.from_mod_index(0), 0);
        assert_eq!(patch_string.from_mod_index(4), 4);


    }

    #[test]
    fn test_combined_operations() {
        let mut patch_string = PatchString::new(String::from("hello world"));
        patch_string.insert(5, " dear"); // Inserts " dear" at position 5
        patch_string.delete(11, 16);     // Deletes "world" (from index 11 to 16)
        // Testing to_mod_index after multiple patches
        assert_eq!(patch_string.to_mod_index(0), 0);
        assert_eq!(patch_string.to_mod_index(5), 10); // Adjusted by the insertion
        assert_eq!(patch_string.to_mod_index(6), 11);
        assert_eq!(patch_string.to_mod_index(11), 11); // End of the deleted range

        // Testing from_mod_index after multiple patches
        assert_eq!(patch_string.from_mod_index(0), 0);
        assert_eq!(patch_string.from_mod_index(10), 5);  // Adjusted by insertion
        // assert_eq!(patch_string.from_mod_index(11), 6);  // Adjusted by both insertion and deletion
    }
}

