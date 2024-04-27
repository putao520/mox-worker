use dashmap::DashMap;
use once_cell::sync::Lazy;
static OFFICES_ASSIGN: Lazy<DashMap<u32, u32>> = Lazy::new(||{
    let m = DashMap::<u32, u32>::new();
    m.insert(3642, 164);
    m.insert(3643, 59);
    m.insert(3644, 59);
    m.insert(3645, 246);
    m.insert(3646, 59);
    m.insert(3647, 246);
    m.insert(3648, 246);
    m.insert(3649, 59);
    m.insert(3651, 59);
    m.insert(3652, 59);
    m.insert(3653, 59);
    m.insert(3655, 59);

    m.insert(3656, 246);
    m.insert(3657, 59);
    m.insert(3658, 164);
    m.insert(3659, 246);
    m.insert(3660, 59);
    m.insert(3661, 59);
    m.insert(3663, 59);
    m.insert(3664, 59);
    m.insert(3665, 59);
    m.insert(3666, 59);

    m.insert(3667, 164);
    m.insert(3669, 59);
    m.insert(3670, 59);
    m.insert(3671, 59);
    m.insert(3672, 59);
    m.insert(3673, 59);
    m.insert(3674, 164);

    // 缺少 香港与澳门2个地区分配规则,因为不知道香港办事处信息
    m
});

// 根据用户所在省份，分配领事办事处
pub fn offices_assign(state_id: u32) -> Option<u32> {
    OFFICES_ASSIGN.get(&state_id).map(|v| *v)
}