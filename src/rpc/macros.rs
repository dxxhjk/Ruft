#[macro_export]
macro_rules! append_entries_request {
    //parameter:&self, entries:Vec<String>
    ($node:expr, $entries: expr) => {
        let mut temp_entries = Vec::new();
        let mut tempEntry = Entry {
                index: $node.commit_index,
                term: $node.current_term,
                command: $entries,
            }
        temp_entries.push(tempEntry);
        let aer_msg = RPCMessage::new(Message::AppendEntriesRequest(AppendEntriesRequest::new(
            $node.current_term,
            $node.rpc.cs.socket_addr,
            $node.server.CS.addr,
            $node.last_applied as usize,
            $node.logs.last().unwrap().term,
            temp_entries,
            $node.commit_index,
        )))
        .unwrap();
        $node.rpc.cs.send_all(&aer_msg).unwrap();
    };
}

#[macro_export]
macro_rules! append_entries_response {
    //parameter:&self, success:bool, leader:SocketAddr
    ($node:expr,
    $success: expr,
    $entries: expr,
    $leader: expr) => {
        let aer_msg = RPCMessage::new(Message::AppendEntriesResponse(AppendEntriesResponse::new(
            $node.rpc.cs.socket_addr,
            $node.logs.last().unwrap().index + 1,
            $node.logs.last().unwrap().index + 1,
            $entries,
            $node.current_term,
            $success,
        )))
        .unwrap();
        $node
            .rpc
            .cs
            .send_to($leader, &aer_msg)
            .unwrap();
    };
}

#[macro_export]
macro_rules! request_vote {
    //parameter:&self (send to all)
    ($node: expr) => {
        let rvr_msg = RPCMessage::new(Message::RequestVoteRequest(RequestVoteRequest::new(
            $node.current_term,
            $node.rpc.cs.socket_addr,
            $node.last_applied as usize,
            $node.logs.last().unwrap().term,
        )))
        .unwrap();
        $node.rpc.cs.send_all(&rvr_msg).unwrap();
    };
}

#[macro_export]
macro_rules! vote_for {
    //parameter:&self, success:bool, candidate: SocketAddr
    ($node: expr, $vote_granted: expr, $candidate: expr) => {
        let rvr_msg = RPCMessage::new(Message::RequestVoteResponse(RequestVoteResponse::new(
            $node.current_term,
            $vote_granted,
        )))
        .unwrap();
        $node
            .rpc
            .cs
            .send_to($candidate, &rvr_msg)
            .unwrap();
    };
}

#[macro_export]
macro_rules! redirect {
    //parameter:&self, success:bool, candidate: SocketAddr
    ($node: expr, $command: expr, $src: expr) => {
        let rvr_msg = RPCMessage::new(Message::Redirect(Redirect::new(
            $command,
            $src,
        )))
        .unwrap();
        $node
            .rpc
            .cs
            .send_to($node.leader_client_addr.unwrap(), &rvr_msg)
            .unwrap();
    };
}
