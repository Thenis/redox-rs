enum MessageType {
    Ping(PingMessage),
    FindNode(FindNodeMessage),
    GetPeers(GetPeersMessage),
    AnnouncePeer(AnnouncePeerMessage),
    GetData(GetDataMessage),
    PutData(PutDataMessage)
}

impl MessageType {
    fn new<B>(decoded: B) -> DhtResult<MessageType> where B: BencodeView {
        
    }
}

struct PingMessage;

struct FindNodeMessage;

struct GetPeersMessage;

struct AnnouncePeerMessage;

struct GetDataMessage;

struct PutDataMessage;