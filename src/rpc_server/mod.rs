use tonic::{Request, Response, Status};

use raft_rpc::raft_service_server::RaftService;
use raft_rpc::*;

#[tonic::async_trait]
pub trait Responder {
    async fn append_entries(
        &self,
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entry: KeyValue,
        leader_commit: u64,
    ) -> Option<u64>;
    async fn request_vote(
        &self,
        term: u64,
        candidate_id: u64,
        last_log_index: u64,
        last_log_term: u64,
    ) -> (u64, bool);
}

pub struct RaftRpcServer<T: Responder + Sync + Send> {
    responder: T,
}

pub mod raft_rpc {
    tonic::include_proto!("raft_rpc");
}

impl<T: Responder + Sync + Send> RaftRpcServer<T> {
    pub fn new(responder: T) -> RaftRpcServer<T> {
        RaftRpcServer {
            responder,
        }
    }
}

#[tonic::async_trait]
impl<T: Responder + Sync + Send + 'static> RaftService for RaftRpcServer<T> {
    async fn append_entries(&self, req: Request<AppendEntriesRequest>) -> Result<Response<AppendEntriesResponse>, Status> {
        let AppendEntriesRequest {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entry,
            leader_commit,
        } = req.into_inner();

        let res = self.responder.append_entries(term, leader_id, prev_log_index, prev_log_term, entry.unwrap(), leader_commit).await;
        match res {
            Some(term) => Ok(Response::new(AppendEntriesResponse {
                term: Some(term),
            })),
            None => Ok(Response::new(AppendEntriesResponse {
                term: None,
            })),
        }
    }

    async fn request_vote(&self, req: Request<RequestVoteRequest>) -> Result<Response<RequestVoteResponse>, Status> {
        let RequestVoteRequest {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        } = req.into_inner();

        let (term, vote_granted) = self.responder.request_vote(term, candidate_id, last_log_index, last_log_term).await;
        Ok(Response::new(RequestVoteResponse {
            term,
            vote_granted,
        }))
    }
}
