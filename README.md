# MoKa Reads Core Library 

## The Core Library of MoKa Reads Software

> This library is under the [GPLv2 License](LICENSE.md)

This library is used a core way to synchronize various MoKa Reads applications/tools that may interact with eachother. 
There are many plans to build fundamental applications in Rust, so allowing for a central way to distribute types 
allows for us to manage different things more easily. 


Currently one of the main types in the library are the MoKa Reads Resources, although this could've been written 
directly into [MoKa-Web](https://github.com/Moka-Reads/MoKa-Web), I found it easier to develop a seperate library to contain 
it, and since there are plans of seperating different aspects of the web service into microservices, this approach seems 
a good step in the right direction. 


> Currently, this library has not been released to [crates.io](https://crates.io), but there is plans to do so once more work 
> has been put in, and I can stop relying on the bleeding edge commits on projects. 


- <i class="fa-solid fa-book"></i> [Documentation](https://moka-reads.github.io/MokaReads-Core/index.html)