trait ParameterList<O> {
    fn parse(&self, req: &Request) -> TokamakResult<O> {
        todo!()
    }
}
impl<A> ParameterList<(A,)> for (&str,) {}
impl<A, B> ParameterList<(A, B)> for (&str, &str) {}
impl<A, B, C> ParameterList<(A, B, C)> for (&str, &str, &str) {}
impl<A, B, C, D> ParameterList<(A, B, C, D)> for (&str, &str, &str, &str) {}
