use tonic::{Request, Response, Status};

use raft_rpc::raft_service_server::RaftService;
use raft_rpc::*;

use std::sync::{Arc, Mutex};

pub trait Responder {
    fn new() -> Self;
    fn append_entries(
        &mut self,
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entry: Option<LogEntry>,
        leader_commit: u64,
    ) -> (u64, bool);
    fn request_vote(
        &mut self,
        term: u64,
        candidate_id: u64,
        last_log_index: u64,
        last_log_term: u64,
    ) -> (u64, bool);
}

pub struct RaftRpcServer<T: Responder> {
    inner: Arc<Mutex<T>>,
}

pub mod raft_rpc {
    tonic::include_proto!("raft_rpc");
}

impl<T: Responder> RaftRpcServer<T> {
    pub fn new() -> RaftRpcServer<T> {
        RaftRpcServer {
            inner: Arc::new(Mutex::new(T::new())),
        }
    }
}

impl<T: Responder + Sync + Send + 'static> RaftRpcServer<T> {
    fn let_me_borrow_your_inner(&self) -> Arc<Mutex<T>> {
        self.inner.clone()
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

        let (term, success) = self.inner.lock().unwrap().append_entries(term, leader_id, prev_log_index, prev_log_term, entry, leader_commit);
        Ok(Response::new(AppendEntriesResponse{
            term, success,
        }))
    }

    async fn request_vote(&self, req: Request<RequestVoteRequest>) -> Result<Response<RequestVoteResponse>, Status> {
        let RequestVoteRequest {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        } = req.into_inner();

        let (term, vote_granted) = self.inner.lock().unwrap().request_vote(term, candidate_id, last_log_index, last_log_term);
        Ok(Response::new(RequestVoteResponse {
            term,
            vote_granted,
        }))
    }
}
