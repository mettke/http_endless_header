use clap::Clap;

#[derive(Clap, Debug, Clone)]
#[clap(author, about, version)]
pub(crate) struct Args {
    #[clap(short, long, arg_enum, case_insensitive(true))]
    pub(crate) format: ImageFormat,
}

#[derive(Clap, PartialEq, Eq, Debug, Clone)]
pub(crate) enum ImageFormat {
    PNG,
    JPEG,
    GIF,
}
