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
            $node.current_term,                     //term
            $node.rpc.cs.socket_addr,               //leader_addr
            $node.server.CS.addr,                   //leader_client_addr
            $node.last_applied as usize,            //prev_log_index
            $node.logs.last().unwrap().term,        //prev_log_term
            temp_entries,                           //entries
            $node.commit_index,                     //leader_commit
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
            $node.rpc.cs.socket_addr,                //socket_addr
            $node.logs.last().unwrap().index + 1,    //next_index
            $node.logs.last().unwrap().index + 1,    //match_index
            $entries,                                //commands
            $node.current_term,                      //term
            $success,                                //success
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
            $node.current_term,                      //term
            $node.rpc.cs.socket_addr,                //candidated_addr
            $node.last_applied as usize,             //last_log_index
            $node.logs.last().unwrap().term,         //last_log_term
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
            $node.current_term,                      //term
            $vote_granted,                           //vote_granted
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
        let red_msg = RPCMessage::new(Message::Redirect(Redirect::new(
            $command,
            $src,
        )))
        .unwrap();
        $node
            .rpc
            .cs
            .send_to($node.leader_client_addr.unwrap(), &red_msg)
            .unwrap();
    };
}
