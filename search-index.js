var searchIndex = {};
searchIndex['rust-bt'] = {"items":[[0,"","rust-bt",""],[0,"bencode","",""],[2,"BenVal","rust-bt::bencode",""],[12,"Int","","",0],[12,"Bytes","","",0],[12,"List","","",0],[12,"Dict","","",0],[4,"BenResult","",""],[10,"new","","",0],[10,"encoded","","",0],[10,"int","","",0],[10,"bytes","","",0],[10,"str","","",0],[10,"list","","",0],[10,"dict","","",0],[0,"torrent","rust-bt",""],[1,"Torrent","rust-bt::torrent",""],[11,"announce","","",1],[11,"announce_list","","",1],[11,"comment","","",1],[11,"created_by","","",1],[11,"creation_date","","",1],[1,"File","",""],[11,"length","","",2],[11,"md5sum","","",2],[11,"path","","",2],[10,"new","","",1],[10,"is_single_file","","",1],[10,"is_multi_file","","",1],[10,"num_files","","",1],[10,"get_file","","",1],[0,"tracker","rust-bt",""],[1,"AnnounceInfo","rust-bt::tracker",""],[11,"interval","","",3],[11,"leechers","","",3],[11,"seeders","","",3],[11,"peers","","",3],[6,"Tracker","",""],[9,"socket_name","","",4],[9,"announce","","",4],[0,"tracker_udp","rust-bt",""],[1,"UdpTracker","rust-bt::tracker_udp",""],[10,"new","","Creates a new UdpTracker object.",5],[10,"socket_name","","",5],[10,"announce","","",5],[0,"upnp","rust-bt",""],[1,"ServiceDesc","rust-bt::upnp","Type used to represent a service description. Service actions can be viewed\nas well as related in and out parameters and service state variables."],[2,"UPnPInterface","","Type used to represent different interfaces available on a network as defined\nby the UPnP specification. This type can be used to get very general information\nabout an interface but also to get access to the services exposed by the interface."],[12,"Root","","",6],[12,"Device","","",6],[12,"Service","","",6],[12,"Identifier","","",6],[4,"StrPos","","A tuple consisting of start and end position."],[10,"parse","","Takes a location corresponding to the root device description page of a UPnPInterface\nas well as the st field of the interface which will always be unique to a service/device.",7],[10,"soap_request","","",7],[10,"find_all","","Sends out a search request for all UPnP interfaces on the specified from_addr.",6],[10,"find_services","","Sends out a search request for all UPnP services on the specified from_addr.",6],[10,"find_devices","","Sends out a search request for all UPnP devices on the specified from_addr.",6],[10,"find_uuid","","Sends out a search request for a specific UPnP interface on the specified from_addr.",6],[10,"is_root","","Check if the current UPnPInterface represents a root device on the network.",6],[10,"is_device","","Check if the current UPnPInterface represents a device on the network.",6],[10,"is_service","","Check if the current UPnPInterface represents a service on the network.",6],[10,"is_identifier","","Check if the current UPnPInterface represents an identifier (uuid) on the network.",6],[10,"name","","Get the device name, service specification, or uuid that this interface\nhas identified with.",6],[10,"version","","Get the device version, service version, or no version (identifiers/root devices)\nthat this interface has identified with.",6],[10,"location","","Get the location of the root device description page corresponding to\nthis UPnPInterface.",6],[10,"st","","Get the search target.",6],[10,"usn","","Get the full Unique Service Name.",6],[10,"service_desc","","Get the service description for this service.",6],[0,"util","rust-bt",""],[2,"Transport","rust-bt::util",""],[12,"TCP","","",8],[12,"UDP","","",8],[12,"HTTP","","",8],[3,"get_net_addrs","","Returns a list of all local IPv4 Addresses."],[3,"get_udp_sock","","Attempts to open a udp connection on addr.\n \nIf the connection is unsuccessful, it will try again up to (attempts - 1)\ntimes, incrementing the port for each attempt."],[3,"get_udp_wait","","The standard wait algorithm defined in the UDP Tracker Protocol."],[3,"gen_peer_id","","Generates a peer id from a base identifier followed by random characters."],[3,"get_transport","","Takes a url and returns the transport type that it specifies."],[3,"get_sockaddr","","Returns the first found DNS entry as a SocketAddr for the specified url."],[3,"get_path","","Returns the path portion of a supplied url."],[3,"get_error","","Used to fill an IoError with a kind and desc, leaving detail empty."]],"paths":[[2,"BenVal"],[1,"Torrent"],[1,"File"],[1,"AnnounceInfo"],[6,"Tracker"],[1,"UdpTracker"],[2,"UPnPInterface"],[1,"ServiceDesc"],[2,"Transport"]]};
initSearch(searchIndex);
