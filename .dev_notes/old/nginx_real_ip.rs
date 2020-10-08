#[derive(Debug)]
pub struct NginxRealIP(IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for NginxRealIP {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.real_ip() {
            Some(ip) => Outcome::Success(NginxRealIP(ip)),
            None => Outcome::Failure((Status::from_code(401).unwrap(), ()))
        }
    }
}
