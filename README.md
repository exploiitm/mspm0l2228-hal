# MSPM0L222X Hardware Abstraction Layer

HAL for TI MSPM0L2228 Mixed Signal Microcontroller - written for ECTF 2026 
by exploiitm.

This crate uses the PAC in [exploiitm/pac](https://github.com/exploiitm/mspm0l2228-pac)

## Example Usage

1. Build OpenOCD from source
	> [!IMPORTANT] 
	> At the time of writing this README the release version of OpenOCD does not have the patch for mspm0


3. Install `arm-none-eabi-objcopy`
4. Setup new project
	```bash
	$ cargo new hal_test; cd hal_test
	$ cat << EOF > runner.sh
	#!/bin/sh
	arm-none-eabi-objcopy -O binary \$1 firmware.bin &&
	openocd -f board/ti_mspm0_launchpad.cfg -c \\
	    "init; halt;
	    flash write_image erase firmware.bin 0x00000000 bin; 
	    reset run; exit;" # remove exit to wait for GDB
	EOF
	$ cat << EOF > memory.x
	MEMORY
	{
	  FLASH (rx)      : ORIGIN = 0x00000000, LENGTH = 0x00040000  /* 256 KB */
	  RAM  (rwx)     : ORIGIN = 0x20200000, LENGTH = 0x00008000  /* 32 KB */
	}
	EOF
	```
5. Configure build
	
	i) `.cargo/config.toml`
   ```toml 
     [build]
     target = "thumbv6m-none-eabi"

     [target.thumbv6m-none-eabi]
     runner = "./runner.sh"
     ```
     ii) `build.rs`
     ```rust
     //! This build script copies the `memory.x` file from the crate root into
	//! a directory where the linker can always find it at build time.
	//! For many projects this is optional, as the linker always searches the
	//! project root directory -- wherever `Cargo.toml` is. However, if you
	//! are using a workspace or have a more complicated build setup, this
	//! build script becomes required. Additionally, by requesting that
	//! Cargo re-run the build script whenever `memory.x` is changed,
	//! updating `memory.x` ensures a rebuild of the application with the
	//! new memory settings.
	//!
	//! The build script also sets the linker flags to tell it which link script to use.

	use std::env;
	use std::fs::File;
	use std::io::Write;
	use std::path::PathBuf;

	fn main() {
	    // Put `memory.x` in our output directory and ensure it's
	    // on the linker search path.
	    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
	    File::create(out.join("memory.x"))
	        .unwrap()
	        .write_all(include_bytes!("memory.x"))
	        .unwrap();
	    println!("cargo:rustc-link-search={}", out.display());

	    // By default, Cargo will re-run a build script whenever
	    // any file in the project changes. By specifying `memory.x`
	    // here, we ensure the build script is only re-run when
	    // `memory.x` is changed.
	    println!("cargo:rerun-if-changed=memory.x");

	    // Specify linker arguments.

		// `--nmagic` is required if memory section addresses are not aligned to 0x10000,
   		// for example the FLASH and RAM sections in your `memory.x`.
	    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
	    println!("cargo:rustc-link-arg=--nmagic");

	    // Set the linker script to the one provided by cortex-m-rt.
	    println!("cargo:rustc-link-arg=-Tlink.x");
	}
     ```
    
  5. Install dependencies
	  ```bash
	  $ cargo add cortex-m --features cortex-m/critical-section-single-core cortex-m-rt cortex-m-semihosting panic-halt embedded-hal
	  $ cargo add --path <path-to-HAL>
	 ```
6. Test out an example 
	```
	$ cp <path-to-HAL>/examples/random_led.rs src/main.rs
	$ cargo r
	```

This specific example flashes the connected board with code for randomly toggling the RGB LED on the LaunchPad.

> [!IMPORTANT] 
> Make sure the shorts are correctly tied if the LEDs don't light up
