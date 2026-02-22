mod vm;

use vm::Helios32;
use vm::assembler;
use vm::registers::RDS;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() > 5 || args.len() < 2 {
        eprintln!("Usage: helios32 <program>.h32 (<output register>) (-f|--float-output)");
        return;
    }

    let output = if args.len() >= 3 {
        match assembler::parse_register(&args[2]) {
            Ok(reg) => reg,
            Err(err) => {
                eprintln!("{err}");
                return;
            },
        }
    } else {
        RDS
    };

    let mut vm = Helios32::new();

    match assembler::assemble_from_path(&args[1]) {
        Ok(path) => if let Err(err) = vm.load_program_from_path(path) {
            eprintln!("{err}");
            return;
        },
        Err(err) => {
            eprintln!("{err}");
            return;
        },
    }
    
    vm.run();

    if args.len() == 4 {
        if ["-f", "--float-output"].contains(&&*args[3]) {
            println!("{:?}", f32::from_bits(vm.registers[output as usize]));
        } else {
            eprintln!("expected -f or --float-output flag");
        }

        return;
    }

    println!("{}", vm.registers[output as usize]);
}
