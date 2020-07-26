// --------
// WARNING
// This is generated code.
// If you need changes, alter the format-identifier_crategen project, not this file.
// --------

impl FormatIdentifier {
    /// FourCC: `AC-3`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 41-43-2D-33 (ASCII value "AC-3") as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to ATSC A/52.
    pub const AC_3: FormatIdentifier = FormatIdentifier(FourCC(*b"AC-3"));

    /// FourCC: `ADFR`, Registered by: _SNPTV_
    /// 
    /// Intention:
    /// 
    /// > Broadcasters members of SNPTV in France will use this RID into a Managed Private UPID structure, as described in SCTE-35,  for advertisement usages
    pub const ADFR: FormatIdentifier = FormatIdentifier(FourCC(*b"ADFR"));

    /// FourCC: `AMCN`, Registered by: _AMC Networks Inc._
    /// 
    /// Intention:
    /// 
    /// > Usage for identifying program and break segmentation of linear broadcast channels.  
    pub const AMCN: FormatIdentifier = FormatIdentifier(FourCC(*b"AMCN"));

    /// FourCC: `ARRS`, Registered by: _Arris Group, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    pub const ARRS: FormatIdentifier = FormatIdentifier(FourCC(*b"ARRS"));

    /// FourCC: `AVSA`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-41 (ASCII value "AVSA") and as the audio elementary stream ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the audio elementary stream loop in the Program Map Table (PMT) to identify each audio elementary stream that conforms to one Audio part of AVS standards.
    pub const AVSA: FormatIdentifier = FormatIdentifier(FourCC(*b"AVSA"));

    /// FourCC: `AVSV`, Registered by: _Audio Video Coding Standard Working Group of China_
    /// 
    /// Intention:
    /// 
    /// > AVS wishes to register the hex value 41-56-53-56 (ASCII value "AVSV") as the video elementary stream ID format_identifier. The AVS standard requires that a registration_descriptor() appear in the video elementary stream loop in the Program Map Table (PMT) to identify each video elementary stream that conforms to one Video part of AVS standards.
    pub const AVSV: FormatIdentifier = FormatIdentifier(FourCC(*b"AVSV"));

    /// FourCC: `BDC0`, Registered by: _Broadcast Data Corporation_
    /// 
    /// Intention:
    /// 
    /// > Broadcast Data Corporation Update TV Software Data Download Service Carousel 0x00
    pub const BDC0: FormatIdentifier = FormatIdentifier(FourCC(*b"BDC0"));

    /// FourCC: `BSSD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Audio data as specified in SMPTE Standard SMPTE 302M-1998 for Television - Mapping of AES3 Data into MPEG-2 Transport Stream shall use this format_identifier.
    pub const BSSD: FormatIdentifier = FormatIdentifier(FourCC(*b"BSSD"));

    /// FourCC: `CAPO`, Registered by: _SMPTE_
    /// 
    /// Intention:
    /// 
    /// > SMPTE 315M-1999 SMPTE STANDARD for Television -- Camera Positioning Information Conveyed by Ancillary Data Packets, Clause 5 - User data words, 5.1 SMPTE universal label (LABEL) - The 16 words UDW 0-15 carry the SMPTE-administered universal label to identify the class of metadata camera positioning information. The 4 words UDW 8-11 are the ISO/IEC 13818-1 (MPEG) registration identifier.
    pub const CAPO: FormatIdentifier = FormatIdentifier(FourCC(*b"CAPO"));

    /// FourCC: `CUEI`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier CUEI in our standard SCTE 35 2003, Digital Program Insertion Cueing Message for Cable
    pub const CUEI: FormatIdentifier = FormatIdentifier(FourCC(*b"CUEI"));

    /// FourCC: `DDED`, Registered by: _LGEUS_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for Digital Delivery with encryption and decryption
    pub const DDED: FormatIdentifier = FormatIdentifier(FourCC(*b"DDED"));

    /// FourCC: `DISC`, Registered by: _DISCOVERY COMMUNICATIONS, LLC._
    /// 
    /// Intention:
    /// 
    /// > Discovery Communications, LLC., hearby request the registration of format_identifier "DISC" to be used as Private Unique Program Identifier as described in SCTE-35
    pub const DISC: FormatIdentifier = FormatIdentifier(FourCC(*b"DISC"));

    /// FourCC: `DISH`, Registered by: _EchoStar Communications Corporation_
    /// 
    /// Intention:
    /// 
    /// > RID will be used to identify MPEG streams that come from EchoStar's broadcast network.
    pub const DISH: FormatIdentifier = FormatIdentifier(FourCC(*b"DISH"));

    /// FourCC: `dmat`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 64 6d 61 74 (ASCII value ³dmat") as the audio elementary stream ID format_identifier for the Dolby MAT audio format. This format is used for delivery of audio and time-aligned metadata between consumer electronics devices.
    pub const DMAT: FormatIdentifier = FormatIdentifier(FourCC(*b"dmat"));

    /// FourCC: `DRA1`, Registered by: _Digital Rise_
    /// 
    /// Intention:
    /// 
    /// > DRA1 is to be used for DRA digital audio standard (Chinese Electronic Industry Standard, Specification for Multichannel Digital Audio Coding Technology SJ/T11368-2006) in broadcast and television industry.
    pub const DRA1: FormatIdentifier = FormatIdentifier(FourCC(*b"DRA1"));

    /// FourCC: `drac`, Registered by: _British Broadcasting Corporation_
    /// 
    /// Intention:
    /// 
    /// > The BBC wishes to use "drac" as the format identifier for the Dirac codec within an MPEG-2 transport stream. The Dirac specification is available publicly at http://dirac.sourceforge.net/specification.html and a mapping specification for use with MPEG-2 Transport Streams is available publicly at http://schrodinger.sourceforge.net/dirac-in-13818-1-mapping.pdf
    pub const DRAC: FormatIdentifier = FormatIdentifier(FourCC(*b"drac"));

    /// FourCC: `DTG1`, Registered by: _Digital TV Group_
    /// 
    /// Intention:
    /// 
    /// > Active region descriptions may be broadcast to describe the portion of the 16:9 or 4:3 coded
    /// > frame that is “of interest”. The region descriptions are informative in nature and are provided to assist receivers to optimise their presentation of video. The active regions definitions are carried in the user data of the video elementary stream.
    pub const DTG1: FormatIdentifier = FormatIdentifier(FourCC(*b"DTG1"));

    /// FourCC: `DTS1`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS1: FormatIdentifier = FormatIdentifier(FourCC(*b"DTS1"));

    /// FourCC: `DTS2`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS2: FormatIdentifier = FormatIdentifier(FourCC(*b"DTS2"));

    /// FourCC: `DTS3`, Registered by: _DTS Inc._
    /// 
    /// Intention:
    /// 
    /// > DTS wishes to register three format_identifiers (DTS1, DTS2, DTS3) to correspond to the three DTS frame sizes of 512, 1024 and 2048.
    pub const DTS3: FormatIdentifier = FormatIdentifier(FourCC(*b"DTS3"));

    /// FourCC: `DTVI`, Registered by: _DTV Innovations_
    /// 
    /// Intention:
    /// 
    /// > We have developed data broadcasting products and want a format identifier to provide a method of identifying the underlying private data.
    pub const DTVI: FormatIdentifier = FormatIdentifier(FourCC(*b"DTVI"));

    /// FourCC: `DVDF`, Registered by: _DVD Format/Logo Licensing Corporation_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (DVDF:MPEG Transport Stream for DVD equipments defined by DVD Forum) is to indicate that stream is in conformity with DVD Stream Format Standard.
    pub const DVDF: FormatIdentifier = FormatIdentifier(FourCC(*b"DVDF"));

    /// FourCC: `EAC3`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used for Insertion, transmission and inspection of private data for various Arris products.
    pub const EAC3: FormatIdentifier = FormatIdentifier(FourCC(*b"EAC3"));

    /// FourCC: `EBP0`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP0: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP0"));

    /// FourCC: `EBP1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP1: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP1"));

    /// FourCC: `EBP2`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP2: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP2"));

    /// FourCC: `EBP3`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP3: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP3"));

    /// FourCC: `EBP4`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP4: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP4"));

    /// FourCC: `EBP5`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP5: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP5"));

    /// FourCC: `EBP6`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP6: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP6"));

    /// FourCC: `EBP7`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP7: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP7"));

    /// FourCC: `EBP8`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP8: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP8"));

    /// FourCC: `EBP9`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > To be used in CableLabs specification OC-SP-EBP-D01-12XXXX
    pub const EBP9: FormatIdentifier = FormatIdentifier(FourCC(*b"EBP9"));

    /// FourCC: `ETV1`, Registered by: _Cable Television Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Cablelabs wishes to register the hex value of 45-54-56-31 (ASCII value "ETV1") as the etv_format_identifier in the registration descriptor. The etv registration descriptor is defined for use in the elementary stream information loop of the PMT. The ETV registration descriptor SHALL be carried in the elementary stream information loop of the PMT for each program component that conveys an ETV integrated signaling stream or ETV application resource stream.
    pub const ETV1: FormatIdentifier = FormatIdentifier(FourCC(*b"ETV1"));

    /// FourCC: `GA94`, Registered by: _Advanced Television Systems Committee_
    /// 
    /// Intention:
    /// 
    /// > The ATSC wishes to register the hex value 47-41-39-34 (ASCII value "GA94") as the program ID format_identifier. The ATSC standard requires that a registration_descriptor() appear in the program loop in the Program Map Table (PMT) to identify each program that conforms to ATSC A/53.
    pub const GA94: FormatIdentifier = FormatIdentifier(FourCC(*b"GA94"));

    /// FourCC: `GWKS`, Registered by: _GuideWorks_
    /// 
    /// Intention:
    /// 
    /// > "GWKS" identifies the stream that contains the interactive program guide data
    pub const GWKS: FormatIdentifier = FormatIdentifier(FourCC(*b"GWKS"));

    /// FourCC: `HDMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > This RID is used for identification of the stream which conforms to the specification of "System Description Blu-ray Disc Read-Only Format part 3 Audio Visual Basic Specifications"
    pub const HDMV: FormatIdentifier = FormatIdentifier(FourCC(*b"HDMV"));

    /// FourCC: `HDMX`, Registered by: _Matsushita Electric Industrial Co. Ltd_
    /// 
    /// Intention:
    /// 
    /// > Matsushita wishes to register the hex value of 48-44-4D-58 (ASCII value "HDMX") “HDMX” identifies the transport stream
    pub const HDMX: FormatIdentifier = FormatIdentifier(FourCC(*b"HDMX"));

    /// FourCC: `HDPR`, Registered by: _Network Business Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "HDPR" indicates that the stream format conforms to our new products
    pub const HDPR: FormatIdentifier = FormatIdentifier(FourCC(*b"HDPR"));

    /// FourCC: `HLIT`, Registered by: _Harmonic Inc._
    /// 
    /// Intention:
    /// 
    /// > This RID will be used to identify coding decisions in bitstreams produced by Harmonic encoders
    pub const HLIT: FormatIdentifier = FormatIdentifier(FourCC(*b"HLIT"));

    /// FourCC: `ID3 `, Registered by: _Organization	Apple, Inc._
    /// 
    /// Intention:
    /// 
    /// > We wish to be able to place ID3 metadata in streams, and we intend to deploy such streams on the public internet at some point.
    pub const ID3 : FormatIdentifier = FormatIdentifier(FourCC(*b"ID3 "));

    /// FourCC: `KLVA`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > KLV Data as specified in SMPTE Recommended Practice RP 217-2001 - Non-synchronized Mapping of KLV Packets into MPEG-2 Systems Streams, shall use this format_identifier.
    pub const KLVA: FormatIdentifier = FormatIdentifier(FourCC(*b"KLVA"));

    /// FourCC: `LIPS`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > For use with ST 2064 family of standards
    pub const LIPS: FormatIdentifier = FormatIdentifier(FourCC(*b"LIPS"));

    /// FourCC: `LU-A`, Registered by: _Harris Corporation_
    /// 
    /// Intention:
    /// 
    /// > The data indicated by this ID is defined in SMPTE RDD-11.
    pub const LU_A: FormatIdentifier = FormatIdentifier(FourCC(*b"LU-A"));

    /// FourCC: `mlpa`, Registered by: _Dolby Laboratories, Inc._
    /// 
    /// Intention:
    /// 
    /// > Dolby Laboratories, Inc. wishes to register the hex value 6d 6c 70 61 (ASCII value "mlpa") as the audio elementary stream ID format_identifier for the MLP audio format (also known as Dolby TrueHD). This format is used for lossless audio compression and delivery.
    pub const MLPA: FormatIdentifier = FormatIdentifier(FourCC(*b"mlpa"));

    /// FourCC: `MTRM`, Registered by: _Victor Company of Japan, Limited_
    /// 
    /// Intention:
    /// 
    /// > The purpose of the assigned RID (MTRM: MPEG Transport Stream for Recording Media) is to indicate that the stream is in conformity with the D-VHS Stream Format Standard.
    pub const MTRM: FormatIdentifier = FormatIdentifier(FourCC(*b"MTRM"));

    /// FourCC: `NBCU`, Registered by: _NBC Universal_
    /// 
    /// Intention:
    /// 
    /// > To be used in SCTE 35 for custom UPIDs and data carriage
    pub const NBCU: FormatIdentifier = FormatIdentifier(FourCC(*b"NBCU"));

    /// FourCC: `NMR1`, Registered by: _Nielsen Media Research_
    /// 
    /// Intention:
    /// 
    /// > Purpose: content identification for audience measurement
    pub const NMR1: FormatIdentifier = FormatIdentifier(FourCC(*b"NMR1"));

    /// FourCC: `NPO1`, Registered by: _Nederlandse Publieke Omroep (NPO, Dutch Public Broadcasting)_
    /// 
    /// Intention:
    /// 
    /// > Requesting a registered private identifier as part of our SCTE-104 and SCTE-35
    /// > implementation, utilizing a managed private UPID. A reserved format_identifier is
    /// > required to comply with SCTE-35 specification, preferred name is NPO1
    pub const NPO1: FormatIdentifier = FormatIdentifier(FourCC(*b"NPO1"));

    /// FourCC: `NWTV`, Registered by: _Digital TV Information Research Group_
    /// 
    /// Intention:
    /// 
    /// > The RID "NWTV" indicates that the stream format conforms to Digital TV Informatization Research Group's specification(http://nw-dtv.jp/)
    pub const NWTV: FormatIdentifier = FormatIdentifier(FourCC(*b"NWTV"));

    /// FourCC: `OMVC`, Registered by: _Open Mobile Video Coalition (OMVC)_
    /// 
    /// Intention:
    /// 
    /// > This value is expected to be used by Mobile DTV broadcasters as part of the ATSC_private_information_descriptor() to scope the syntax and semantics of private data.
    pub const OMVC: FormatIdentifier = FormatIdentifier(FourCC(*b"OMVC"));

    /// FourCC: `Opus`, Registered by: _Mozilla_
    /// 
    /// Intention:
    /// 
    /// > We intend to see this RID applied in various Open Source projects for the carriage of Opus audio in MPEG-TS
    pub const OPUS: FormatIdentifier = FormatIdentifier(FourCC(*b"Opus"));

    /// FourCC: `PAUX`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PRMC). PAUX identifies an elementary stream carrying low-speed data.
    pub const PAUX: FormatIdentifier = FormatIdentifier(FourCC(*b"PAUX"));

    /// FourCC: `PMSF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Partially Modified Stream Format Identifier (PMSF) is to identify the stream which is a Transport Stream modified partially by a DSM Digital Storage Media). Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    pub const PMSF: FormatIdentifier = FormatIdentifier(FourCC(*b"PMSF"));

    /// FourCC: `PRMC`, Registered by: _Philips DVS_
    /// 
    /// Intention:
    /// 
    /// > Philips DVS wishes to apply two format identifers in one of its products (see also PAUX). PRMC identifies an elementary stream carrying remote control data.
    pub const PRMC: FormatIdentifier = FormatIdentifier(FourCC(*b"PRMC"));

    /// FourCC: `PXSA`, Registered by: _Proximus_
    /// 
    /// Intention:
    /// 
    /// > ProximusTV hereby requests the format_identifier 'PXMA' to be used as private Unique Program Identifier, as described in SCTE-35. 
    pub const PXSA: FormatIdentifier = FormatIdentifier(FourCC(*b"PXSA"));

    /// FourCC: `RTLN`, Registered by: _RTL Nederland_
    /// 
    /// Intention:
    /// 
    /// > RTL Nederland hereby requests the format_identifier 'RTLN' to be used as private Unique Program Identifier, as described in SCTE-35.
    pub const RTLN: FormatIdentifier = FormatIdentifier(FourCC(*b"RTLN"));

    /// FourCC: `SBSB`, Registered by: _SBS Broadcasting_
    /// 
    /// Intention:
    /// 
    /// > SBS Broadcasting hereby requests the format_identifier 'SBSB' to be used as private Unique Program Identifier, as described in SCTE-35
    pub const SBSB: FormatIdentifier = FormatIdentifier(FourCC(*b"SBSB"));

    /// FourCC: `SCTE`, Registered by: _Society of Cable Telecommunications Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: We intend to utilize the format identifier SCTE in our standard SCTE 54 2003, Digital Video Service Multiplex and Transport System for Cable Television
    pub const SCTE: FormatIdentifier = FormatIdentifier(FourCC(*b"SCTE"));

    /// FourCC: `SEN1`, Registered by: _Sencore_
    /// 
    /// Intention:
    /// 
    /// > The format_identifier SEN1 is used in an ATSC Private Information Descriptor to identify the source of a stream. It can be embedded in the VANC of an SDI signal in accordance with SMPTE RP207.
    pub const SEN1: FormatIdentifier = FormatIdentifier(FourCC(*b"SEN1"));

    /// FourCC: `SESF`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > Self-Encoded Stream Format identifier (SESF) is to identify the stream which conforms to "System Description Blu-ray Disc ReWritable Format part 3 Audio Visual Basic Specifications". Digital TV or Set Top Box is able to identify this kind of stream comes from a digital interface such as IEEE1394.
    pub const SESF: FormatIdentifier = FormatIdentifier(FourCC(*b"SESF"));

    /// FourCC: `SOPI`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "SOPI" is used to insert data to be used for private information to Sony products.
    pub const SOPI: FormatIdentifier = FormatIdentifier(FourCC(*b"SOPI"));

    /// FourCC: `SPLC`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_splice_format_identifier to identify transport streams complying with SMPTE 312M - Proposed SMPTE Standard for Television - Splice Points for MPEG-2 Transport Streams
    pub const SPLC: FormatIdentifier = FormatIdentifier(FourCC(*b"SPLC"));

    /// FourCC: `SVMD`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > SMPTE_Video_Metadata_Dictionary_identifier to identify transport streams complying with SMPTE xxxM - Proposed SMPTE Standard for Television - Video Metatdata Dictionary for MPEG-2 Transport Streams
    pub const SVMD: FormatIdentifier = FormatIdentifier(FourCC(*b"SVMD"));

    /// FourCC: `SYNC`, Registered by: _Syncbak, Inc._
    /// 
    /// Intention:
    /// 
    /// > The RID will be used to identify Syncbak data broadcasting streams.
    pub const SYNC: FormatIdentifier = FormatIdentifier(FourCC(*b"SYNC"));

    /// FourCC: `SZMI`, Registered by: _Building B, Inc_
    /// 
    /// Intention:
    /// 
    /// > Building B, Inc. is launching a USA-based home entertainment service. Certain service-related data will be broadcast to subscribers homes by embedding the data into ATSC broadcasts. Local digital television stations will be partners with Building B in the deployment of this service. As specified in the ATSC digital television standard A/53, Building B will identify all private data carried in the ATSC stream using MPEG-2 Registration Descriptors and/or ATSC private information descriptors, both of which use the RID "SZMI"
    pub const SZMI: FormatIdentifier = FormatIdentifier(FourCC(*b"SZMI"));

    /// FourCC: `TRIV`, Registered by: _Triveni Digital_
    /// 
    /// Intention:
    /// 
    /// > Purpose: Format identifier for ATSC Private Information Descriptor
    pub const TRIV: FormatIdentifier = FormatIdentifier(FourCC(*b"TRIV"));

    /// FourCC: `TSBV`, Registered by: _Toshiba Corporation Digital Media Network Company_
    /// 
    /// Intention:
    /// 
    /// > "TSBV" identifies the transport stream that contains self-encoded MPEG4-AVC/H.264 data and private data
    pub const TSBV: FormatIdentifier = FormatIdentifier(FourCC(*b"TSBV"));

    /// FourCC: `TSHV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSHV" identifies the transport stream that contains self-encoded MPEG data and private data
    pub const TSHV: FormatIdentifier = FormatIdentifier(FourCC(*b"TSHV"));

    /// FourCC: `TSMV`, Registered by: _Sony Corporation_
    /// 
    /// Intention:
    /// 
    /// > "TSMV" identifies the transport stream that contains self-encoded MPEG data and private data
    pub const TSMV: FormatIdentifier = FormatIdentifier(FourCC(*b"TSMV"));

    /// FourCC: `TTA0`, Registered by: _Telecommunication Technology Association(TTA)_
    /// 
    /// Intention:
    /// 
    /// > TTA (Telecommunications Technology Association), a standard development organization in Korea, establishes and provides technical standards that reflect the latest domestic and international technological advances in IT industry. Especially, TTA Terrestrial Broadcasting Project Group (PG802) is response for the terrestrial broadcasting standards with relevance to ATSC. Recently, TTA PG802 has been working in standardization of advanced codec which describes the compression method of video/audio signal and the transmission/signaling method of the compressed stream using advanced codec in terrestrial digital television broadcasting based on ATSC A/71, 72. In order to rule out the possibility of existing legacy television's malfunction caused by unrecognized error of parameterized service and to designate service type unambiguously, we would like to utilize ATSC_Private_Information_Descriptor, which will be used only in Korea, with new format_identifie r value. Thus we request for the registration of new format_identifier "TTA0" (54-54-41-30 in Hexadecimal Value) because TTA is responsible for establishing standards for terrestrial digital television in Korea.
    pub const TTA0: FormatIdentifier = FormatIdentifier(FourCC(*b"TTA0"));

    /// FourCC: `TVG1`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    pub const TVG1: FormatIdentifier = FormatIdentifier(FourCC(*b"TVG1"));

    /// FourCC: `TVG2`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for two format identifiers [TVG1, TVG2] that identify Electronic Program Guide related data.
    pub const TVG2: FormatIdentifier = FormatIdentifier(FourCC(*b"TVG2"));

    /// FourCC: `TVG3`, Registered by: _Rovi Corporation_
    /// 
    /// Intention:
    /// 
    /// > TV Guide On Screen wishes to apply for a format identifier [TVG3] that identifies Electronic Program Guide related data.
    pub const TVG3: FormatIdentifier = FormatIdentifier(FourCC(*b"TVG3"));

    /// FourCC: `ULE1`, Registered by: _University of Aberdeen (on behalf of the Internet Engineering Task Force, IETF)_
    /// 
    /// Intention:
    /// 
    /// > This application is submitted on behalf of the IETF ipdvb Working Group. The request is to allocate a format_identifier value within the SMPTE RA, such as "ULE1" to describe transport streams carrying data in a format specified by "The Unidirectional Lightweight Encapsulation, ULE". This is an IETF Standards-Track protocol designed to ease support of IPv4, IPv6, MPLS, and various other forms of network packet in data networks built using MPEG-2 Transport Streams. ULE is specified in RFC4326, which is published as an Internet Standards-Track document in the RFC-series. The official reference citation is: [RFC4326] Fairhurst, G. and B. Collini-Nocker, "Unidirectional Lightweight Encapsulation (ULE) for Transmission of IP Datagrams over an MPEG-2 Transport Stream (TS)", RFC 4326, December 2005. The stable reference to this document is maintained at: http://www.ietf.org/rfc/rfc4326.txt. Format identifiers indicating ULE streams may be placed in the PMT ES_info descriptor loop.
    pub const ULE1: FormatIdentifier = FormatIdentifier(FourCC(*b"ULE1"));

    /// FourCC: `ULI0`, Registered by: _Update Logic, Inc._
    /// 
    /// Intention:
    /// 
    /// > Update Logic Inc intends to use "ULI0" as the format identifier for a Data Carousel Service
    pub const ULI0: FormatIdentifier = FormatIdentifier(FourCC(*b"ULI0"));

    /// FourCC: `VC-1`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-1 per SMPTE Draft RP 227 - VC-1 Bitstream Transport Encodings
    pub const VC_1: FormatIdentifier = FormatIdentifier(FourCC(*b"VC-1"));

    /// FourCC: `VC-4`, Registered by: _Society of Motion Picture and Television Engineers_
    /// 
    /// Intention:
    /// 
    /// > Purpose: To provide a format_identifier for VC-4 per SMPTE RP 2058-3 Bitstream Transport Encodings
    pub const VC_4: FormatIdentifier = FormatIdentifier(FourCC(*b"VC-4"));

    /// FourCC: `VMNU`, Registered by: _Viacom_
    /// 
    /// Intention:
    /// 
    /// > We are requesting a registered identifier as part of our implementation of the SCTE-104 and SCTE-35 specifications. Our systems will utilize a managed private UPID as described in SCTE-35. A reserved SMPTE format_identifier is required to comply with this specification.
    pub const VMNU: FormatIdentifier = FormatIdentifier(FourCC(*b"VMNU"));

    /// FourCC: `XMP_`, Registered by: _Adobe Systems_
    /// 
    /// Intention:
    /// 
    /// > Adobe wants to embed XMP metadata into MPEG-2 files to enable file based partner workflows and related standards.
    pub const XMP_: FormatIdentifier = FormatIdentifier(FourCC(*b"XMP_"));

}