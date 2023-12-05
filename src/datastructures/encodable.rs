pub trait Encodable {

    fn encode(&self) -> String;

    fn decode(string: String) -> Self;

}