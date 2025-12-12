fn main() {
	
	println!("Current Shift Force: {}", calculate_shift_force(500, 900))
}
/// Calculate the shift force
fn calculate_shift_force(current_torque: u32, max_torque: u32) -> u32 {
	let force = (current_torque as f32 / max_torque as f32) * 100f32;
	
	// Clamp between 0 and 100
	f32::floor(force.clamp(0f32, 100f32)) as u32
}