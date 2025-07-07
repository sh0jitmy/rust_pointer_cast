use zerocopy::{TryFromBytes,IntoBytes};
use zerocopy_derive::{TryFromBytes,Immutable,IntoBytes,KnownLayout};


#[derive(Debug,TryFromBytes,IntoBytes,KnownLayout,Immutable)]
#[repr(C)]
struct OriginalPacketHead {
	id: u16,
	length: u16,
	value: u16,
}

#[derive(Debug,TryFromBytes,IntoBytes,KnownLayout,Immutable)]
#[repr(C)]
struct OriginalPacket {
	oph: OriginalPacketHead,
	data: [u8; 512],
}

fn main() {
	let mut hbuf = [0u8; std::mem::size_of::<OriginalPacketHead>()];
	
	let oph = OriginalPacketHead::try_mut_from_bytes(&mut hbuf).unwrap();

	oph.id = 0x0001u16;
	oph.length = 2u16;
	oph.value = 0xFFFFu16;

	println!("Original Packet Header: {:?}",oph);
	println!("Raw data:  {:?}",oph.as_bytes());

	let mut pbuf = [0u8; std::mem::size_of::<OriginalPacket>()];
	let opp = OriginalPacket::try_mut_from_bytes(&mut pbuf).unwrap();

	opp.oph.id = 0x0002u16;
	opp.oph.length = 2u16;
	opp.oph.value = 3u16;

	opp.data = [0xff;512];
	
	println!("Original Packet : {:?}",opp);
	println!("Raw data:  {:?}",opp.as_bytes());


}