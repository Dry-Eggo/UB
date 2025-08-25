#![allow(non_snake_case)]
use crate::memory::arena :: {Ptr, self};

pub struct Session {
    allocator:  *mut arena::Allocator,
    inputFile:  String,
    outputFile: Option<String>,
}

fn initSession(allocator: *mut arena::Allocator) -> Ptr<Session> {
    unsafe {
	let mut sess: Ptr<Session> = (*allocator).get_ty();
	sess.get_and(|x| { x.allocator = allocator; });
	sess
    }
}

fn printUsage(name: &str) {
    println!("usage: {name} <input> <output?>");
}

fn parseArguments(mut sess: Ptr<Session>) {
    let args = std::env::args().collect::<Vec<String>>();
    let programName = &args[0];
    if args.len() < 2 {
	printUsage(programName);
	std::process::exit(1);
    }

    let took =  sess.get_and(|x| {
	x.inputFile = args[1].clone();
	if args.len() > 2 {
	    x.outputFile = Some(args[2].clone());
	    return true
	}
	return false
    });

    
}

pub fn beginSession(mut allocator: arena::Allocator) {
    let mut sess = initSession(&mut allocator);
    parseArguments(sess.clone());
}
