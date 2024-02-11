pub fn get_threshold_count(peer_count: usize) -> usize {
    if peer_count <= 2 {
        1
    } else {
        (peer_count * 2) / 3
    }
}
