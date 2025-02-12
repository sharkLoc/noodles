mod header;

use bytes::BytesMut;
use tokio::io::{self, AsyncRead, AsyncReadExt};

use self::header::read_header;
use crate::{
    container::{Container, Header},
    io::reader::container::{get_compression_header, read_slice},
};

pub async fn read_container<R>(reader: &mut R, buf: &mut BytesMut) -> io::Result<Option<Container>>
where
    R: AsyncRead + Unpin,
{
    let mut header = Header::default();

    let len = match read_header(reader, &mut header).await? {
        0 => return Ok(None),
        n => n,
    };

    buf.resize(len, 0);
    reader.read_exact(buf).await?;
    let mut buf = buf.split().freeze();

    let compression_header = get_compression_header(&mut buf)?;

    let slice_count = header.landmarks().len();
    let mut slices = Vec::with_capacity(slice_count);

    for _ in 0..slice_count {
        let slice = read_slice(&mut buf)?;
        slices.push(slice);
    }

    Ok(Some(Container::new(compression_header, slices)))
}
