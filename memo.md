## RPC
- AppendEntries
  - term, log entryを含む
  - Leaderが受け取った場合: 
    - termが自分より新しければfollowerになる
    - 古ければ無視?
  - Followerが受け取った場合:
    - termが自分より古ければ無視
    - そうでなければterm <- log_term, queue.append(log), last_log_index++
  - candidateが受け取った場合:
    - 自分より新しいか同じであればfollowerと同じ処理をしてfollowerに戻る
- RequestVote
  - term, candidate_id, last_log_index, last_log_indexを含む
  - Leaderが受け取った場合:
    - 無視
  - followerが受け取った場合:
    - my_voted_for == voted_for, my_last_log_index <= 


## 設計
- Leader, Follower, Candidateの3つのstate == structを持つ
  - `persistent_state`を持つ
    - `!Send`, `!Clone`
  - `::new()`で`persistent_state`を受け渡す(move)
- `trait RaftRpcResponder`
  - `AppendEntries()`, `RequestVote()`, `InstallSnapshot()`の3つのインターフェースを満たす
  - 全ての関数について`-> impl RaftRpcResponder + HttpReqResponder`もしくは`-> Box<dyn RaftRpcResponder + HttpReqResponder>`
- `trait HttpReqResponder`
  - `HttpAppendEntry()`のみ
  - `/append_entry`へのハンドラ

👆 これでよくないか？
