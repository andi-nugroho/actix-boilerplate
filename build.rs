fn main() {
    println!("cargo:rerun-if-changed=schema.prisma");
    
    #[cfg(not(feature = "mock"))]
    prisma_client_rust::generate();
} 