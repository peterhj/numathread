/*
Copyright 2017 the numadiff authors

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

extern crate libc;

use libc::*;

use std::mem;

pub struct NumaThreadExt;

impl NumaThreadExt {
  //#[cfg(os = "linux")]
  pub fn set_affinity(cpus: &[usize]) -> Result<(), ()> {
    let mut cpuset: cpu_set_t = unsafe { mem::zeroed() };
    unsafe { CPU_ZERO(&mut cpuset) };
    for &cpu in cpus {
      unsafe { CPU_SET(cpu, &mut cpuset) };
    }
    let tid = unsafe { pthread_self() };
    let res = unsafe { pthread_setaffinity_np(tid, mem::size_of::<cpu_set_t>(), &cpuset as *const cpu_set_t) };
    // TODO: check return value.
    Ok(())
  }
}
