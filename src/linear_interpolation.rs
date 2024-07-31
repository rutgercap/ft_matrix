fn lerp<V>(u: V, v: V, t: f32) -> V {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_works() {
        let result = lerp(0., 1., 0.5);

        assert_eq!(result, 0.5);
    }
}
