/// _Format Identifier_ values recorded by the SMPTE Registration Authority
#[derive(Debug, Clone, Copy)]
pub enum FormatIdentifier {
    /// FourCC: `AC-3`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 41-43-2D-33 (ASCII value "AC-3") as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to ATSC A/52.
    Ac3,
    /// FourCC: `ADFR`, Registered by: _SNPTV_
    /// 
    /// Intention:
    /// 
    /// > Broadcasters members of SNPTV in France will use this RID into a Managed Private UPID structure, as described in SCTE-35,  for advertisement usages
    Adfr,
    /// FourCC: `AMCN`, Registered by: _AMC Networks Inc._
    /// 
    /// Intention:
    /// 
    /// > Usage for identifying program and break segmentation of linear broadcast channels.  
    Amcn,
    /// FourCC: `ARRS`, Registered by: _Arris Group, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    Arrs,
    /// FourCC: `AVSA`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-41 (ASCII value "AVSA") and as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to one Audio part of AVS standards.
    Avsa,
    /// FourCC: `AVSV`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-56 (ASCII value "AVSV") as the video elementary stream ID format_identifier. The AVS standard requires that a registration_descriptor() appear in the video elementary stream loop in the Program Map Table (PMT) to identify each video elementary stream that conforms to one Video part of AVS standards.
    Avsv,
    /// FourCC: `BDC0`, Registered by: _Broadcast Data Corporation_
    /// 
    /// Intention:
    /// 
    /// > Broadcast Data Corporation Update TV Software Data Download Service Carousel 0x00
    Bdc0,
    /// FourCC: `BSSD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Audio data as specified in SMPTE Standard SMPTE 302M-1998 for Television - Mapping of AES3 Data into MPEG-2 Transport Stream shall use this format_identifier.
    Bssd,
    /// FourCC: `CAPO`, Registered by: _SMPTE_
    /// 
    /// Intention:
    /// 
    /// > SMPTE 315M-1999 SMPTE STANDARD for Television -- Camera Positioning Information Conveyed by Ancillary Data Packets, Clause 5 - User data words, 5.1 SMPTE universal label (LABEL) - The 16 words UDW 0-15 carry the SMPTE-administered universal label to identify the class of metadata camera positioning information. The 4 words UDW 8-11 are the ISO/IEC 13818-1 (MPEG) registration identifier.
    Capo,
    /// FourCC: `CUEI`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier CUEI in our standard SCTE 35 2003, Digital Program Insertion Cueing Message for Cable
    Cuei,
    /// FourCC: `DDED`, Registered by: _LGEUS_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for Digital Delivery with encryption and decryption
    Dded,
    /// FourCC: `DISC`, Registered by: _DISCOVERY COMMUNICATIONS, LLC._
    /// 
    /// Intention:
    /// 
    /// > Discovery Communications, LLC., hearby request the registration of format_identifier "DISC" to be used as Private Unique Program Identifier as described in SCTE-35
    Disc,
    /// FourCC: `DISH`, Registered by: _EchoStar Communications Corporation_
    /// 
    /// Intention:
    /// 
    /// > RID will be used to identify MPEG streams that come from EchoStar's broadcast network.
    Dish,
    /// FourCC: `dmat`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 64 6d 61 74 (ASCII value ³dmat") as the audio elementary stream ID format_identifier for the Dolby MAT audio format. This format is used for delivery of audio and time-aligned metadata between consumer electronics devices.
    Dmat,
    /// FourCC: `DRA1`, Registered by: _Digital Rise_
    /// 
    /// Intention:
    /// 
    /// > DRA1 is to be used for DRA digital audio standard (Chinese Electronic Industry Standard, Specification for Multichannel Digital Audio Coding Technology SJ/T11368-2006) in broadcast and television industry.
    Dra1,
    /// FourCC: `drac`, Registered by: _British Broadcasting Corporation_
    /// 
    /// Intention:
    /// 
    /// > The BBC wishes to use "drac" as the format identifier for the Dirac codec within an MPEG-2 transport stream. The Dirac specification is available publicly at http://dirac.sourceforge.net/specification.html and a mapping specification for use with MPEG-2 Transport Streams is available publicly at http://schrodinger.sourceforge.net/dirac-in-13818-1-mapping.pdf
    Drac,
    /// FourCC: `DTG1`, Registered by: _Digital TV Group_
    /// 
    /// Intention:
    /// 
    /// > Active region descriptions may be broadcast to describe the portion of the 16:9 or 4:3 coded
    /// > frame that is “of interest”. The region descriptions are informative in nature and are provided to assist receivers to optimise their presentation of video. The active regions definitions are carried in the user data of the video elementary stream.
    Dtg1,
    /// FourCC: `DTS1`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    Dts1,
    /// FourCC: `DTS2`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    Dts2,
    /// FourCC: `DTS3`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    Dts3,
    /// FourCC: `DTVI`, Registered by: _DTV Innovations_
    /// 
    /// Intention:
    /// 
    /// > We have developed data broadcasting products and want a format identifier to provide a method of identifying the underlying private data.
    Dtvi,
    /// FourCC: `DVDF`, Registered by: _DVD Format/Logo Licensing Corporation_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (DVDF:MPEG Transport Stream for DVD equipments defined by DVD Forum) is to indicate that stream is in conformity with DVD Stream Format Standard.
    Dvdf,
    /// FourCC: `EAC3`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    Eac3,
    /// FourCC: `EBP0`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp0,
    /// FourCC: `EBP1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp1,
    /// FourCC: `EBP2`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp2,
    /// FourCC: `EBP3`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp3,
    /// FourCC: `EBP4`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp4,
    /// FourCC: `EBP5`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp5,
    /// FourCC: `EBP6`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp6,
    /// FourCC: `EBP7`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp7,
    /// FourCC: `EBP8`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp8,
    /// FourCC: `EBP9`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    Ebp9,
    /// FourCC: `ETV1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Cablelabs wishes to register the hex value of 45-54-56-31 (ASCII value "ETV1") as the etv_format_identifier in the registration descriptor. The etv registration descriptor is defined for use in the elementary stream information loop of the PMT. The ETV registration descriptor SHALL be carried in the elementary stream information loop of the PMT for each program component that conveys an ETV integrated signaling stream or ETV application resource stream.
    Etv1,
    /// FourCC: `GA94`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 47-41-39-34 (ASCII value "GA94") as the program ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the program loop in the Program Map Table (PMT) to identify each program that conforms to ATSC A/53.
    Ga94,
    /// FourCC: `GWKS`, Registered by: _GuideWorks_
    /// 
    /// Intention:
    /// 
    /// > "GWKS" identifies the stream that contains the interactive program guide data
    Gwks,
    /// FourCC: `HDMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for identification of the stream which conforms to the specification of "System Description Blu-ray Disc Read-Only Format part 3 Audio Visual Basic Specifications"
    Hdmv,
    /// FourCC: `HDMX`, Registered by: _Matsushita Electric Industrial Co. Ltd_
    /// 
    /// Intention:
    /// 
    /// > Matsushita wishes to register the hex value of 48-44-4D-58 (ASCII value "HDMX") “HDMX” identifies the transport stream
    Hdmx,
    /// FourCC: `HDPR`, Registered by: _Network Business Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "HDPR" indicates that the stream format conforms to our new products
    Hdpr,
    /// FourCC: `HLIT`, Registered by: _Harmonic Inc._
    /// 
    /// Intention:
    /// 
    /// > This RID will be used to identify coding decisions in bitstreams produced by Harmonic encoders
    Hlit,
    /// FourCC: `ID3 `, Registered by: _Organization	Apple, Inc._
    /// 
    /// Intention:
    /// 
    /// > We wish to be able to place ID3 metadata in streams, and we intend to deploy such streams on the public internet at some point.
    Id3,
    /// FourCC: `KLVA`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > KLV Data as specified in SMPTE Recommended Practice RP 217-2001 - Non-synchronized Mapping of KLV Packets into MPEG-2 Systems Streams, shall use this format_identifier.
    Klva,
    /// FourCC: `LIPS`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > For use with ST 2064 family of standards
    Lips,
    /// FourCC: `LU-A`, Registered by: _Harris Corporation_
    /// 
    /// Intention:
    /// 
    /// > The data indicated by this ID is defined in SMPTE RDD-11.
    LuA,
    /// FourCC: `mlpa`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 6d 6c 70 61 (ASCII value "mlpa") as the audio elementary stream ID format_identifier for the MLP audio format (also known as Dolby TrueHD). This format is used for lossless audio compression and delivery.
    Mlpa,
    /// FourCC: `MTRM`, Registered by: _Victor Company of Japan, Limited_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (MTRM: MPEG Transport Stream for Recording Media) is to indicate that the stream is in conformity with the D-VHS Stream Format Standard.
    Mtrm,
    /// FourCC: `NBCU`, Registered by: _NBC Universal_
    /// 
    /// Intention:
    /// 
    /// > To be used in SCTE 35 for custom UPIDs and data carriage
    Nbcu,
    /// FourCC: `NMR1`, Registered by: _Nielsen Media Research_
    /// 
    /// Intention:
    /// 
    /// > Purpose: content identification for audience measurement
    Nmr1,
    /// FourCC: `NPO1`, Registered by: _Nederlandse Publieke Omroep (NPO, Dutch Public Broadcasting)_
    /// 
    /// Intention:
    /// 
    /// > Requesting a registered private identifier as part of our SCTE-104 and SCTE-35
    /// > implementation, utilizing a managed private UPID. A reserved format_identifier is
    /// > required to comply with SCTE-35 specification, preferred name is NPO1
    Npo1,
    /// FourCC: `NWTV`, Registered by: _Digital TV Information Research Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "NWTV" indicates that the stream format conforms to Digital TV Informatization Research Group's specification(http://nw-dtv.jp/)
    Nwtv,
    /// FourCC: `OMVC`, Registered by: _Open Mobile Video Coalition (OMVC)_
    /// 
    /// Intention:
    /// 
    /// > This value is expected to be used by Mobile DTV broadcasters as part of the ATSC_private_information_descriptor() to scope the syntax and semantics of private data.
    Omvc,
    /// FourCC: `Opus`, Registered by: _Mozilla_
    /// 
    /// Intention:
    /// 
    /// > We intend to see this RID applied in various Open Source projects for the carriage of Opus audio in MPEG-TS
    Opus,
    /// FourCC: `PAUX`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PRMC). PAUX identifies an elementary stream carrying low-speed data.
    Paux,
    /// FourCC: `PMSF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Partially Modified Stream Format Identifier (PMSF) is to identify the stream which is a Transport Stream modified partially by a DSM Digital Storage Media). Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    Pmsf,
    /// FourCC: `PRMC`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PAUX). PRMC identifies an elementary stream carrying remote control data.
    Prmc,
    /// FourCC: `PXSA`, Registered by: _Proximus_
    /// 
    /// Intention:
    /// 
    /// > ProximusTV hereby requests the format_identifier 'PXMA' to be used as private Unique Program Identifier, as described in SCTE-35. 
    Pxsa,
    /// FourCC: `RTLN`, Registered by: _RTL Nederland_
    /// 
    /// Intention:
    /// 
    /// > RTL Nederland hereby requests the format_identifier 'RTLN' to be used as private Unique Program Identifier, as described in SCTE-35.
    Rtln,
    /// FourCC: `SBSB`, Registered by: _SBS Broadcasting_
    /// 
    /// Intention:
    /// 
    /// > SBS Broadcasting hereby requests the format_identifier 'SBSB' to be used as private Unique Program Identifier, as described in SCTE-35
    Sbsb,
    /// FourCC: `SCTE`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier SCTE in our standard SCTE 54 2003, Digital Video Service Multiplex and Transport System for Cable Television
    Scte,
    /// FourCC: `SEN1`, Registered by: _Sencore_
    /// 
    /// Intention:
    /// 
    /// > The format_identifier SEN1 is used in an ATSC Private Information Descriptor to identify the source of a stream. It can be embedded in the VANC of an SDI signal in accordance with SMPTE RP207.
    Sen1,
    /// FourCC: `SESF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Self-Encoded Stream Format identifier (SESF) is to identify the stream which conforms to "System Description Blu-ray Disc ReWritable Format part 3 Audio Visual Basic Specifications". Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    Sesf,
    /// FourCC: `SOPI`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "SOPI" is used to insert data to be used for private information to Sony products.
    Sopi,
    /// FourCC: `SPLC`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_splice_format_identifier to identify transport streams complying with SMPTE 312M - Proposed SMPTE Standard for Television - Splice Points for MPEG-2 Transport Streams
    Splc,
    /// FourCC: `SVMD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_Video_Metadata_Dictionary_identifier to identify transport streams complying with SMPTE xxxM - Proposed SMPTE Standard for Television - Video Metatdata Dictionary for MPEG-2 Transport Streams
    Svmd,
    /// FourCC: `SYNC`, Registered by: _Syncbak, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used to identify Syncbak data broadcasting streams.
    Sync,
    /// FourCC: `SZMI`, Registered by: _Building B, Inc_
    /// 
    /// Intention:
    /// 
    /// > Building B, Inc. is launching a USA-based home entertainment service. Certain service-related data will be broadcast to subscribers homes by embedding the data into ATSC broadcasts. Local digital television stations will be partners with Building B in the deployment of this service. As specified in the ATSC digital television standard A/53, Building B will identify all private data carried in the ATSC stream using MPEG-2 Registration Descriptors and/or ATSC private information descriptors, both of which use the RID "SZMI"
    Szmi,
    /// FourCC: `TRIV`, Registered by: _Triveni Digital_
    /// 
    /// Intention:
    /// 
    /// > Purpose: Format identifier for ATSC Private Information Descriptor
    Triv,
    /// FourCC: `TSBV`, Registered by: _Toshiba Corporation Digital Media Network Company_
    /// 
    /// Intention:
    /// 
    /// > "TSBV" identifies the transport stream that contains self-encoded MPEG4-AVC/H.264 data and private data
    Tsbv,
    /// FourCC: `TSHV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSHV" identifies the transport stream that contains self-encoded MPEG data and private data
    Tshv,
    /// FourCC: `TSMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSMV" identifies the transport stream that contains self-encoded MPEG data and private data
    Tsmv,
    /// FourCC: `TTA0`, Registered by: _Telecommunication Technology Association(TTA)_
    /// 
    /// Intention:
    /// 
    /// > TTA (Telecommunications Technology Association), a standard development organization in Korea, establishes and provides technical standards that reflect the latest domestic and international technological advances in IT industry. Especially, TTA Terrestrial Broadcasting Project Group (PG802) is response for the terrestrial broadcasting standards with relevance to ATSC. Recently, TTA PG802 has been working in standardization of advanced codec which describes the compression method of video/audio signal and the transmission/signaling method of the compressed stream using advanced codec in terrestrial digital television broadcasting based on ATSC A/71, 72. In order to rule out the possibility of existing legacy television's malfunction caused by unrecognized error of parameterized service and to designate service type unambiguously, we would like to utilize ATSC_Private_Information_Descriptor, which will be used only in Korea, with new format_identifie r value. Thus we request for the registration of new format_identifier "TTA0" (54-54-41-30 in Hexadecimal Value) because TTA is responsible for establishing standards for terrestrial digital television in Korea.
    Tta0,
    /// FourCC: `TVG1`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    Tvg1,
    /// FourCC: `TVG2`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    Tvg2,
    /// FourCC: `TVG3`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for a format identifier [TVG3] that identifies Electronic Program Guide related data.
    Tvg3,
    /// FourCC: `ULE1`, Registered by: _University of Aberdeen (on behalf of the Internet Engineering Task Force, IETF)_
    /// 
    /// Intention:
    /// 
    /// > This application is submitted on behalf of the IETF ipdvb Working Group. The request is to allocate a format_identifier value within the SMPTE RA, such as "ULE1" to describe transport streams carrying data in a format specified by "The Unidirectional Lightweight Encapsulation, ULE". This is an IETF Standards-Track protocol designed to ease support of IPv4, IPv6, MPLS, and various other forms of network packet in data networks built using MPEG-2 Transport Streams. ULE is specified in RFC4326, which is published as an Internet Standards-Track document in the RFC-series. The official reference citation is: [RFC4326] Fairhurst, G. and B. Collini-Nocker, "Unidirectional Lightweight Encapsulation (ULE) for Transmission of IP Datagrams over an MPEG-2 Transport Stream (TS)", RFC 4326, December 2005. The stable reference to this document is maintained at: http://www.ietf.org/rfc/rfc4326.txt. Format identifiers indicating ULE streams may be placed in the PMT ES_info descriptor loop.
    Ule1,
    /// FourCC: `ULI0`, Registered by: _Update Logic, Inc._
    /// 
    /// Intention:
    /// 
    /// > Update Logic Inc intends to use "ULI0" as the format identifier for a Data Carousel Service
    Uli0,
    /// FourCC: `VC-1`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-1 per SMPTE Draft RP 227 - VC-1 Bitstream Transport Encodings
    Vc1,
    /// FourCC: `VC-4`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-4 per SMPTE RP 2058-3 Bitstream Transport Encodings
    Vc4,
    /// FourCC: `VMNU`, Registered by: _Viacom_
    /// 
    /// Intention:
    /// 
    /// > We are requesting a registered identifier as part of our implementation of the SCTE-104 and SCTE-35 specifications. Our systems will utilize a managed private UPID as described in SCTE-35. A reserved SMPTE format_identifier is required to comply with this specification.
    Vmnu,
    /// FourCC: `XMP_`, Registered by: _Adobe Systems_
    /// 
    /// Intention:
    /// 
    /// > Adobe wants to embed XMP metadata into MPEG-2 files to enable file based partner workflows and related standards.
    Xmp,
    Unknown(four_cc::FourCC),
}

impl From<four_cc::FourCC> for FormatIdentifier {
    fn from(val: four_cc::FourCC) -> Self {
        match val {
            format_identifier::AC_3 => FormatIdentifier::Ac3,
            format_identifier::ADFR => FormatIdentifier::Adfr,
            format_identifier::AMCN => FormatIdentifier::Amcn,
            format_identifier::ARRS => FormatIdentifier::Arrs,
            format_identifier::AVSA => FormatIdentifier::Avsa,
            format_identifier::AVSV => FormatIdentifier::Avsv,
            format_identifier::BDC0 => FormatIdentifier::Bdc0,
            format_identifier::BSSD => FormatIdentifier::Bssd,
            format_identifier::CAPO => FormatIdentifier::Capo,
            format_identifier::CUEI => FormatIdentifier::Cuei,
            format_identifier::DDED => FormatIdentifier::Dded,
            format_identifier::DISC => FormatIdentifier::Disc,
            format_identifier::DISH => FormatIdentifier::Dish,
            format_identifier::DMAT => FormatIdentifier::Dmat,
            format_identifier::DRA1 => FormatIdentifier::Dra1,
            format_identifier::DRAC => FormatIdentifier::Drac,
            format_identifier::DTG1 => FormatIdentifier::Dtg1,
            format_identifier::DTS1 => FormatIdentifier::Dts1,
            format_identifier::DTS2 => FormatIdentifier::Dts2,
            format_identifier::DTS3 => FormatIdentifier::Dts3,
            format_identifier::DTVI => FormatIdentifier::Dtvi,
            format_identifier::DVDF => FormatIdentifier::Dvdf,
            format_identifier::EAC3 => FormatIdentifier::Eac3,
            format_identifier::EBP0 => FormatIdentifier::Ebp0,
            format_identifier::EBP1 => FormatIdentifier::Ebp1,
            format_identifier::EBP2 => FormatIdentifier::Ebp2,
            format_identifier::EBP3 => FormatIdentifier::Ebp3,
            format_identifier::EBP4 => FormatIdentifier::Ebp4,
            format_identifier::EBP5 => FormatIdentifier::Ebp5,
            format_identifier::EBP6 => FormatIdentifier::Ebp6,
            format_identifier::EBP7 => FormatIdentifier::Ebp7,
            format_identifier::EBP8 => FormatIdentifier::Ebp8,
            format_identifier::EBP9 => FormatIdentifier::Ebp9,
            format_identifier::ETV1 => FormatIdentifier::Etv1,
            format_identifier::GA94 => FormatIdentifier::Ga94,
            format_identifier::GWKS => FormatIdentifier::Gwks,
            format_identifier::HDMV => FormatIdentifier::Hdmv,
            format_identifier::HDMX => FormatIdentifier::Hdmx,
            format_identifier::HDPR => FormatIdentifier::Hdpr,
            format_identifier::HLIT => FormatIdentifier::Hlit,
            format_identifier::ID3  => FormatIdentifier::Id3,
            format_identifier::KLVA => FormatIdentifier::Klva,
            format_identifier::LIPS => FormatIdentifier::Lips,
            format_identifier::LU_A => FormatIdentifier::LuA,
            format_identifier::MLPA => FormatIdentifier::Mlpa,
            format_identifier::MTRM => FormatIdentifier::Mtrm,
            format_identifier::NBCU => FormatIdentifier::Nbcu,
            format_identifier::NMR1 => FormatIdentifier::Nmr1,
            format_identifier::NPO1 => FormatIdentifier::Npo1,
            format_identifier::NWTV => FormatIdentifier::Nwtv,
            format_identifier::OMVC => FormatIdentifier::Omvc,
            format_identifier::OPUS => FormatIdentifier::Opus,
            format_identifier::PAUX => FormatIdentifier::Paux,
            format_identifier::PMSF => FormatIdentifier::Pmsf,
            format_identifier::PRMC => FormatIdentifier::Prmc,
            format_identifier::PXSA => FormatIdentifier::Pxsa,
            format_identifier::RTLN => FormatIdentifier::Rtln,
            format_identifier::SBSB => FormatIdentifier::Sbsb,
            format_identifier::SCTE => FormatIdentifier::Scte,
            format_identifier::SEN1 => FormatIdentifier::Sen1,
            format_identifier::SESF => FormatIdentifier::Sesf,
            format_identifier::SOPI => FormatIdentifier::Sopi,
            format_identifier::SPLC => FormatIdentifier::Splc,
            format_identifier::SVMD => FormatIdentifier::Svmd,
            format_identifier::SYNC => FormatIdentifier::Sync,
            format_identifier::SZMI => FormatIdentifier::Szmi,
            format_identifier::TRIV => FormatIdentifier::Triv,
            format_identifier::TSBV => FormatIdentifier::Tsbv,
            format_identifier::TSHV => FormatIdentifier::Tshv,
            format_identifier::TSMV => FormatIdentifier::Tsmv,
            format_identifier::TTA0 => FormatIdentifier::Tta0,
            format_identifier::TVG1 => FormatIdentifier::Tvg1,
            format_identifier::TVG2 => FormatIdentifier::Tvg2,
            format_identifier::TVG3 => FormatIdentifier::Tvg3,
            format_identifier::ULE1 => FormatIdentifier::Ule1,
            format_identifier::ULI0 => FormatIdentifier::Uli0,
            format_identifier::VC_1 => FormatIdentifier::Vc1,
            format_identifier::VC_4 => FormatIdentifier::Vc4,
            format_identifier::VMNU => FormatIdentifier::Vmnu,
            format_identifier::XMP_ => FormatIdentifier::Xmp,
            _ => FormatIdentifier::Unknown(val),
        }
    }
}

/// `FourCC`'s for _Format Identifiers_ recorded by the SMPTE Registration Authority
pub mod format_identifier {
    /// FourCC: `AC-3`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 41-43-2D-33 (ASCII value "AC-3") as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to ATSC A/52.
    pub const AC_3: four_cc::FourCC = four_cc::FourCC(*b"AC-3");

    /// FourCC: `ADFR`, Registered by: _SNPTV_
    /// 
    /// Intention:
    /// 
    /// > Broadcasters members of SNPTV in France will use this RID into a Managed Private UPID structure, as described in SCTE-35,  for advertisement usages
    pub const ADFR: four_cc::FourCC = four_cc::FourCC(*b"ADFR");

    /// FourCC: `AMCN`, Registered by: _AMC Networks Inc._
    /// 
    /// Intention:
    /// 
    /// > Usage for identifying program and break segmentation of linear broadcast channels.  
    pub const AMCN: four_cc::FourCC = four_cc::FourCC(*b"AMCN");

    /// FourCC: `ARRS`, Registered by: _Arris Group, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    pub const ARRS: four_cc::FourCC = four_cc::FourCC(*b"ARRS");

    /// FourCC: `AVSA`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-41 (ASCII value "AVSA") and as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to one Audio part of AVS standards.
    pub const AVSA: four_cc::FourCC = four_cc::FourCC(*b"AVSA");

    /// FourCC: `AVSV`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-56 (ASCII value "AVSV") as the video elementary stream ID format_identifier. The AVS standard requires that a registration_descriptor() appear in the video elementary stream loop in the Program Map Table (PMT) to identify each video elementary stream that conforms to one Video part of AVS standards.
    pub const AVSV: four_cc::FourCC = four_cc::FourCC(*b"AVSV");

    /// FourCC: `BDC0`, Registered by: _Broadcast Data Corporation_
    /// 
    /// Intention:
    /// 
    /// > Broadcast Data Corporation Update TV Software Data Download Service Carousel 0x00
    pub const BDC0: four_cc::FourCC = four_cc::FourCC(*b"BDC0");

    /// FourCC: `BSSD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Audio data as specified in SMPTE Standard SMPTE 302M-1998 for Television - Mapping of AES3 Data into MPEG-2 Transport Stream shall use this format_identifier.
    pub const BSSD: four_cc::FourCC = four_cc::FourCC(*b"BSSD");

    /// FourCC: `CAPO`, Registered by: _SMPTE_
    /// 
    /// Intention:
    /// 
    /// > SMPTE 315M-1999 SMPTE STANDARD for Television -- Camera Positioning Information Conveyed by Ancillary Data Packets, Clause 5 - User data words, 5.1 SMPTE universal label (LABEL) - The 16 words UDW 0-15 carry the SMPTE-administered universal label to identify the class of metadata camera positioning information. The 4 words UDW 8-11 are the ISO/IEC 13818-1 (MPEG) registration identifier.
    pub const CAPO: four_cc::FourCC = four_cc::FourCC(*b"CAPO");

    /// FourCC: `CUEI`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier CUEI in our standard SCTE 35 2003, Digital Program Insertion Cueing Message for Cable
    pub const CUEI: four_cc::FourCC = four_cc::FourCC(*b"CUEI");

    /// FourCC: `DDED`, Registered by: _LGEUS_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for Digital Delivery with encryption and decryption
    pub const DDED: four_cc::FourCC = four_cc::FourCC(*b"DDED");

    /// FourCC: `DISC`, Registered by: _DISCOVERY COMMUNICATIONS, LLC._
    /// 
    /// Intention:
    /// 
    /// > Discovery Communications, LLC., hearby request the registration of format_identifier "DISC" to be used as Private Unique Program Identifier as described in SCTE-35
    pub const DISC: four_cc::FourCC = four_cc::FourCC(*b"DISC");

    /// FourCC: `DISH`, Registered by: _EchoStar Communications Corporation_
    /// 
    /// Intention:
    /// 
    /// > RID will be used to identify MPEG streams that come from EchoStar's broadcast network.
    pub const DISH: four_cc::FourCC = four_cc::FourCC(*b"DISH");

    /// FourCC: `dmat`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 64 6d 61 74 (ASCII value ³dmat") as the audio elementary stream ID format_identifier for the Dolby MAT audio format. This format is used for delivery of audio and time-aligned metadata between consumer electronics devices.
    pub const DMAT: four_cc::FourCC = four_cc::FourCC(*b"dmat");

    /// FourCC: `DRA1`, Registered by: _Digital Rise_
    /// 
    /// Intention:
    /// 
    /// > DRA1 is to be used for DRA digital audio standard (Chinese Electronic Industry Standard, Specification for Multichannel Digital Audio Coding Technology SJ/T11368-2006) in broadcast and television industry.
    pub const DRA1: four_cc::FourCC = four_cc::FourCC(*b"DRA1");

    /// FourCC: `drac`, Registered by: _British Broadcasting Corporation_
    /// 
    /// Intention:
    /// 
    /// > The BBC wishes to use "drac" as the format identifier for the Dirac codec within an MPEG-2 transport stream. The Dirac specification is available publicly at http://dirac.sourceforge.net/specification.html and a mapping specification for use with MPEG-2 Transport Streams is available publicly at http://schrodinger.sourceforge.net/dirac-in-13818-1-mapping.pdf
    pub const DRAC: four_cc::FourCC = four_cc::FourCC(*b"drac");

    /// FourCC: `DTG1`, Registered by: _Digital TV Group_
    /// 
    /// Intention:
    /// 
    /// > Active region descriptions may be broadcast to describe the portion of the 16:9 or 4:3 coded
    /// > frame that is “of interest”. The region descriptions are informative in nature and are provided to assist receivers to optimise their presentation of video. The active regions definitions are carried in the user data of the video elementary stream.
    pub const DTG1: four_cc::FourCC = four_cc::FourCC(*b"DTG1");

    /// FourCC: `DTS1`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS1: four_cc::FourCC = four_cc::FourCC(*b"DTS1");

    /// FourCC: `DTS2`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS2: four_cc::FourCC = four_cc::FourCC(*b"DTS2");

    /// FourCC: `DTS3`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS3: four_cc::FourCC = four_cc::FourCC(*b"DTS3");

    /// FourCC: `DTVI`, Registered by: _DTV Innovations_
    /// 
    /// Intention:
    /// 
    /// > We have developed data broadcasting products and want a format identifier to provide a method of identifying the underlying private data.
    pub const DTVI: four_cc::FourCC = four_cc::FourCC(*b"DTVI");

    /// FourCC: `DVDF`, Registered by: _DVD Format/Logo Licensing Corporation_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (DVDF:MPEG Transport Stream for DVD equipments defined by DVD Forum) is to indicate that stream is in conformity with DVD Stream Format Standard.
    pub const DVDF: four_cc::FourCC = four_cc::FourCC(*b"DVDF");

    /// FourCC: `EAC3`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    pub const EAC3: four_cc::FourCC = four_cc::FourCC(*b"EAC3");

    /// FourCC: `EBP0`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP0: four_cc::FourCC = four_cc::FourCC(*b"EBP0");

    /// FourCC: `EBP1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP1: four_cc::FourCC = four_cc::FourCC(*b"EBP1");

    /// FourCC: `EBP2`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP2: four_cc::FourCC = four_cc::FourCC(*b"EBP2");

    /// FourCC: `EBP3`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP3: four_cc::FourCC = four_cc::FourCC(*b"EBP3");

    /// FourCC: `EBP4`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP4: four_cc::FourCC = four_cc::FourCC(*b"EBP4");

    /// FourCC: `EBP5`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP5: four_cc::FourCC = four_cc::FourCC(*b"EBP5");

    /// FourCC: `EBP6`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP6: four_cc::FourCC = four_cc::FourCC(*b"EBP6");

    /// FourCC: `EBP7`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP7: four_cc::FourCC = four_cc::FourCC(*b"EBP7");

    /// FourCC: `EBP8`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP8: four_cc::FourCC = four_cc::FourCC(*b"EBP8");

    /// FourCC: `EBP9`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP9: four_cc::FourCC = four_cc::FourCC(*b"EBP9");

    /// FourCC: `ETV1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Cablelabs wishes to register the hex value of 45-54-56-31 (ASCII value "ETV1") as the etv_format_identifier in the registration descriptor. The etv registration descriptor is defined for use in the elementary stream information loop of the PMT. The ETV registration descriptor SHALL be carried in the elementary stream information loop of the PMT for each program component that conveys an ETV integrated signaling stream or ETV application resource stream.
    pub const ETV1: four_cc::FourCC = four_cc::FourCC(*b"ETV1");

    /// FourCC: `GA94`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 47-41-39-34 (ASCII value "GA94") as the program ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the program loop in the Program Map Table (PMT) to identify each program that conforms to ATSC A/53.
    pub const GA94: four_cc::FourCC = four_cc::FourCC(*b"GA94");

    /// FourCC: `GWKS`, Registered by: _GuideWorks_
    /// 
    /// Intention:
    /// 
    /// > "GWKS" identifies the stream that contains the interactive program guide data
    pub const GWKS: four_cc::FourCC = four_cc::FourCC(*b"GWKS");

    /// FourCC: `HDMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for identification of the stream which conforms to the specification of "System Description Blu-ray Disc Read-Only Format part 3 Audio Visual Basic Specifications"
    pub const HDMV: four_cc::FourCC = four_cc::FourCC(*b"HDMV");

    /// FourCC: `HDMX`, Registered by: _Matsushita Electric Industrial Co. Ltd_
    /// 
    /// Intention:
    /// 
    /// > Matsushita wishes to register the hex value of 48-44-4D-58 (ASCII value "HDMX") “HDMX” identifies the transport stream
    pub const HDMX: four_cc::FourCC = four_cc::FourCC(*b"HDMX");

    /// FourCC: `HDPR`, Registered by: _Network Business Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "HDPR" indicates that the stream format conforms to our new products
    pub const HDPR: four_cc::FourCC = four_cc::FourCC(*b"HDPR");

    /// FourCC: `HLIT`, Registered by: _Harmonic Inc._
    /// 
    /// Intention:
    /// 
    /// > This RID will be used to identify coding decisions in bitstreams produced by Harmonic encoders
    pub const HLIT: four_cc::FourCC = four_cc::FourCC(*b"HLIT");

    /// FourCC: `ID3 `, Registered by: _Organization	Apple, Inc._
    /// 
    /// Intention:
    /// 
    /// > We wish to be able to place ID3 metadata in streams, and we intend to deploy such streams on the public internet at some point.
    pub const ID3 : four_cc::FourCC = four_cc::FourCC(*b"ID3 ");

    /// FourCC: `KLVA`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > KLV Data as specified in SMPTE Recommended Practice RP 217-2001 - Non-synchronized Mapping of KLV Packets into MPEG-2 Systems Streams, shall use this format_identifier.
    pub const KLVA: four_cc::FourCC = four_cc::FourCC(*b"KLVA");

    /// FourCC: `LIPS`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > For use with ST 2064 family of standards
    pub const LIPS: four_cc::FourCC = four_cc::FourCC(*b"LIPS");

    /// FourCC: `LU-A`, Registered by: _Harris Corporation_
    /// 
    /// Intention:
    /// 
    /// > The data indicated by this ID is defined in SMPTE RDD-11.
    pub const LU_A: four_cc::FourCC = four_cc::FourCC(*b"LU-A");

    /// FourCC: `mlpa`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 6d 6c 70 61 (ASCII value "mlpa") as the audio elementary stream ID format_identifier for the MLP audio format (also known as Dolby TrueHD). This format is used for lossless audio compression and delivery.
    pub const MLPA: four_cc::FourCC = four_cc::FourCC(*b"mlpa");

    /// FourCC: `MTRM`, Registered by: _Victor Company of Japan, Limited_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (MTRM: MPEG Transport Stream for Recording Media) is to indicate that the stream is in conformity with the D-VHS Stream Format Standard.
    pub const MTRM: four_cc::FourCC = four_cc::FourCC(*b"MTRM");

    /// FourCC: `NBCU`, Registered by: _NBC Universal_
    /// 
    /// Intention:
    /// 
    /// > To be used in SCTE 35 for custom UPIDs and data carriage
    pub const NBCU: four_cc::FourCC = four_cc::FourCC(*b"NBCU");

    /// FourCC: `NMR1`, Registered by: _Nielsen Media Research_
    /// 
    /// Intention:
    /// 
    /// > Purpose: content identification for audience measurement
    pub const NMR1: four_cc::FourCC = four_cc::FourCC(*b"NMR1");

    /// FourCC: `NPO1`, Registered by: _Nederlandse Publieke Omroep (NPO, Dutch Public Broadcasting)_
    /// 
    /// Intention:
    /// 
    /// > Requesting a registered private identifier as part of our SCTE-104 and SCTE-35
    /// > implementation, utilizing a managed private UPID. A reserved format_identifier is
    /// > required to comply with SCTE-35 specification, preferred name is NPO1
    pub const NPO1: four_cc::FourCC = four_cc::FourCC(*b"NPO1");

    /// FourCC: `NWTV`, Registered by: _Digital TV Information Research Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "NWTV" indicates that the stream format conforms to Digital TV Informatization Research Group's specification(http://nw-dtv.jp/)
    pub const NWTV: four_cc::FourCC = four_cc::FourCC(*b"NWTV");

    /// FourCC: `OMVC`, Registered by: _Open Mobile Video Coalition (OMVC)_
    /// 
    /// Intention:
    /// 
    /// > This value is expected to be used by Mobile DTV broadcasters as part of the ATSC_private_information_descriptor() to scope the syntax and semantics of private data.
    pub const OMVC: four_cc::FourCC = four_cc::FourCC(*b"OMVC");

    /// FourCC: `Opus`, Registered by: _Mozilla_
    /// 
    /// Intention:
    /// 
    /// > We intend to see this RID applied in various Open Source projects for the carriage of Opus audio in MPEG-TS
    pub const OPUS: four_cc::FourCC = four_cc::FourCC(*b"Opus");

    /// FourCC: `PAUX`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PRMC). PAUX identifies an elementary stream carrying low-speed data.
    pub const PAUX: four_cc::FourCC = four_cc::FourCC(*b"PAUX");

    /// FourCC: `PMSF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Partially Modified Stream Format Identifier (PMSF) is to identify the stream which is a Transport Stream modified partially by a DSM Digital Storage Media). Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    pub const PMSF: four_cc::FourCC = four_cc::FourCC(*b"PMSF");

    /// FourCC: `PRMC`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PAUX). PRMC identifies an elementary stream carrying remote control data.
    pub const PRMC: four_cc::FourCC = four_cc::FourCC(*b"PRMC");

    /// FourCC: `PXSA`, Registered by: _Proximus_
    /// 
    /// Intention:
    /// 
    /// > ProximusTV hereby requests the format_identifier 'PXMA' to be used as private Unique Program Identifier, as described in SCTE-35. 
    pub const PXSA: four_cc::FourCC = four_cc::FourCC(*b"PXSA");

    /// FourCC: `RTLN`, Registered by: _RTL Nederland_
    /// 
    /// Intention:
    /// 
    /// > RTL Nederland hereby requests the format_identifier 'RTLN' to be used as private Unique Program Identifier, as described in SCTE-35.
    pub const RTLN: four_cc::FourCC = four_cc::FourCC(*b"RTLN");

    /// FourCC: `SBSB`, Registered by: _SBS Broadcasting_
    /// 
    /// Intention:
    /// 
    /// > SBS Broadcasting hereby requests the format_identifier 'SBSB' to be used as private Unique Program Identifier, as described in SCTE-35
    pub const SBSB: four_cc::FourCC = four_cc::FourCC(*b"SBSB");

    /// FourCC: `SCTE`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier SCTE in our standard SCTE 54 2003, Digital Video Service Multiplex and Transport System for Cable Television
    pub const SCTE: four_cc::FourCC = four_cc::FourCC(*b"SCTE");

    /// FourCC: `SEN1`, Registered by: _Sencore_
    /// 
    /// Intention:
    /// 
    /// > The format_identifier SEN1 is used in an ATSC Private Information Descriptor to identify the source of a stream. It can be embedded in the VANC of an SDI signal in accordance with SMPTE RP207.
    pub const SEN1: four_cc::FourCC = four_cc::FourCC(*b"SEN1");

    /// FourCC: `SESF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Self-Encoded Stream Format identifier (SESF) is to identify the stream which conforms to "System Description Blu-ray Disc ReWritable Format part 3 Audio Visual Basic Specifications". Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    pub const SESF: four_cc::FourCC = four_cc::FourCC(*b"SESF");

    /// FourCC: `SOPI`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "SOPI" is used to insert data to be used for private information to Sony products.
    pub const SOPI: four_cc::FourCC = four_cc::FourCC(*b"SOPI");

    /// FourCC: `SPLC`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_splice_format_identifier to identify transport streams complying with SMPTE 312M - Proposed SMPTE Standard for Television - Splice Points for MPEG-2 Transport Streams
    pub const SPLC: four_cc::FourCC = four_cc::FourCC(*b"SPLC");

    /// FourCC: `SVMD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_Video_Metadata_Dictionary_identifier to identify transport streams complying with SMPTE xxxM - Proposed SMPTE Standard for Television - Video Metatdata Dictionary for MPEG-2 Transport Streams
    pub const SVMD: four_cc::FourCC = four_cc::FourCC(*b"SVMD");

    /// FourCC: `SYNC`, Registered by: _Syncbak, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used to identify Syncbak data broadcasting streams.
    pub const SYNC: four_cc::FourCC = four_cc::FourCC(*b"SYNC");

    /// FourCC: `SZMI`, Registered by: _Building B, Inc_
    /// 
    /// Intention:
    /// 
    /// > Building B, Inc. is launching a USA-based home entertainment service. Certain service-related data will be broadcast to subscribers homes by embedding the data into ATSC broadcasts. Local digital television stations will be partners with Building B in the deployment of this service. As specified in the ATSC digital television standard A/53, Building B will identify all private data carried in the ATSC stream using MPEG-2 Registration Descriptors and/or ATSC private information descriptors, both of which use the RID "SZMI"
    pub const SZMI: four_cc::FourCC = four_cc::FourCC(*b"SZMI");

    /// FourCC: `TRIV`, Registered by: _Triveni Digital_
    /// 
    /// Intention:
    /// 
    /// > Purpose: Format identifier for ATSC Private Information Descriptor
    pub const TRIV: four_cc::FourCC = four_cc::FourCC(*b"TRIV");

    /// FourCC: `TSBV`, Registered by: _Toshiba Corporation Digital Media Network Company_
    /// 
    /// Intention:
    /// 
    /// > "TSBV" identifies the transport stream that contains self-encoded MPEG4-AVC/H.264 data and private data
    pub const TSBV: four_cc::FourCC = four_cc::FourCC(*b"TSBV");

    /// FourCC: `TSHV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSHV" identifies the transport stream that contains self-encoded MPEG data and private data
    pub const TSHV: four_cc::FourCC = four_cc::FourCC(*b"TSHV");

    /// FourCC: `TSMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSMV" identifies the transport stream that contains self-encoded MPEG data and private data
    pub const TSMV: four_cc::FourCC = four_cc::FourCC(*b"TSMV");

    /// FourCC: `TTA0`, Registered by: _Telecommunication Technology Association(TTA)_
    /// 
    /// Intention:
    /// 
    /// > TTA (Telecommunications Technology Association), a standard development organization in Korea, establishes and provides technical standards that reflect the latest domestic and international technological advances in IT industry. Especially, TTA Terrestrial Broadcasting Project Group (PG802) is response for the terrestrial broadcasting standards with relevance to ATSC. Recently, TTA PG802 has been working in standardization of advanced codec which describes the compression method of video/audio signal and the transmission/signaling method of the compressed stream using advanced codec in terrestrial digital television broadcasting based on ATSC A/71, 72. In order to rule out the possibility of existing legacy television's malfunction caused by unrecognized error of parameterized service and to designate service type unambiguously, we would like to utilize ATSC_Private_Information_Descriptor, which will be used only in Korea, with new format_identifie r value. Thus we request for the registration of new format_identifier "TTA0" (54-54-41-30 in Hexadecimal Value) because TTA is responsible for establishing standards for terrestrial digital television in Korea.
    pub const TTA0: four_cc::FourCC = four_cc::FourCC(*b"TTA0");

    /// FourCC: `TVG1`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    pub const TVG1: four_cc::FourCC = four_cc::FourCC(*b"TVG1");

    /// FourCC: `TVG2`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    pub const TVG2: four_cc::FourCC = four_cc::FourCC(*b"TVG2");

    /// FourCC: `TVG3`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for a format identifier [TVG3] that identifies Electronic Program Guide related data.
    pub const TVG3: four_cc::FourCC = four_cc::FourCC(*b"TVG3");

    /// FourCC: `ULE1`, Registered by: _University of Aberdeen (on behalf of the Internet Engineering Task Force, IETF)_
    /// 
    /// Intention:
    /// 
    /// > This application is submitted on behalf of the IETF ipdvb Working Group. The request is to allocate a format_identifier value within the SMPTE RA, such as "ULE1" to describe transport streams carrying data in a format specified by "The Unidirectional Lightweight Encapsulation, ULE". This is an IETF Standards-Track protocol designed to ease support of IPv4, IPv6, MPLS, and various other forms of network packet in data networks built using MPEG-2 Transport Streams. ULE is specified in RFC4326, which is published as an Internet Standards-Track document in the RFC-series. The official reference citation is: [RFC4326] Fairhurst, G. and B. Collini-Nocker, "Unidirectional Lightweight Encapsulation (ULE) for Transmission of IP Datagrams over an MPEG-2 Transport Stream (TS)", RFC 4326, December 2005. The stable reference to this document is maintained at: http://www.ietf.org/rfc/rfc4326.txt. Format identifiers indicating ULE streams may be placed in the PMT ES_info descriptor loop.
    pub const ULE1: four_cc::FourCC = four_cc::FourCC(*b"ULE1");

    /// FourCC: `ULI0`, Registered by: _Update Logic, Inc._
    /// 
    /// Intention:
    /// 
    /// > Update Logic Inc intends to use "ULI0" as the format identifier for a Data Carousel Service
    pub const ULI0: four_cc::FourCC = four_cc::FourCC(*b"ULI0");

    /// FourCC: `VC-1`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-1 per SMPTE Draft RP 227 - VC-1 Bitstream Transport Encodings
    pub const VC_1: four_cc::FourCC = four_cc::FourCC(*b"VC-1");

    /// FourCC: `VC-4`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-4 per SMPTE RP 2058-3 Bitstream Transport Encodings
    pub const VC_4: four_cc::FourCC = four_cc::FourCC(*b"VC-4");

    /// FourCC: `VMNU`, Registered by: _Viacom_
    /// 
    /// Intention:
    /// 
    /// > We are requesting a registered identifier as part of our implementation of the SCTE-104 and SCTE-35 specifications. Our systems will utilize a managed private UPID as described in SCTE-35. A reserved SMPTE format_identifier is required to comply with this specification.
    pub const VMNU: four_cc::FourCC = four_cc::FourCC(*b"VMNU");

    /// FourCC: `XMP_`, Registered by: _Adobe Systems_
    /// 
    /// Intention:
    /// 
    /// > Adobe wants to embed XMP metadata into MPEG-2 files to enable file based partner workflows and related standards.
    pub const XMP_: four_cc::FourCC = four_cc::FourCC(*b"XMP_");
}