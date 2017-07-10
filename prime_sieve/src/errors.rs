error_chain!{
    foreign_links {
       IoError(::std::io::Error);
       ParseInt(::std::num::ParseIntError);
    }
}