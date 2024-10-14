/*
   Copyright 2021 Oliver Giersch

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

#[macro_use]
mod doc;

macro_rules! impl_clone {
    () => {
        #[inline]
        fn clone(&self) -> Self {
            Self { inner: self.inner, _marker: PhantomData }
        }
    };
}

macro_rules! impl_debug {
    ($type_name:expr) => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let (ptr, tag) = self.decompose();
            f.debug_struct($type_name).field("ptr", &ptr).field("tag", &tag).finish()
        }
    };
}

macro_rules! impl_default {
    () => {
        #[inline]
        fn default() -> Self {
            Self::null()
        }
    };
}

macro_rules! impl_pointer {
    () => {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Pointer::fmt(&self.decompose_ptr(), f)
        }
    };
}

macro_rules! impl_partial_eq {
    () => {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.inner.eq(&other.inner)
        }
    };
}

macro_rules! impl_partial_ord {
    () => {
        #[inline]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.inner.partial_cmp(&other.inner)
        }
    };
}

macro_rules! impl_ord {
    () => {
        #[inline]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.inner.cmp(&other.inner)
        }
    };
}

macro_rules! impl_hash {
    () => {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.inner.hash(state)
        }
    };
}
