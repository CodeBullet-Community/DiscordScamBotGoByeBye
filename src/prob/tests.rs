//commenting all this out because it currently tests dead code
///// testing that a https link doesn't give a false positive
//#[test]
//fn https_link(){
//
//    assert!(super::calc_content_spam_prob("https://example.com here's some other stuff")<0.1);
//}
///// testing that a http link doesn't give a false positive
//#[test]
//fn http_link(){
//    assert!(super::calc_content_spam_prob("http://example.com other stuff") < 0.1);
//}
//
///// testing just an everyone ping doesn't give a false positive
//#[test]
//fn everyone_ping(){
//    assert!(super::calc_content_spam_prob("@everyone hey how are you doing?")<0.9);
//}
//
///// testing an everyone ping and https link triggers spam probability
//#[test]
//fn https_and_everyone(){
//    assert!(super::calc_content_spam_prob("https://example.com @everyone")>0.9);
//}
//
///// testing an everyone ping and http link triggers spam probability
//#[test]
//fn http_and_everyone(){
//    assert!(super::calc_content_spam_prob("http://example.com @everyone")>0.9);
//}
///// testing the probability calculation won't give a false positive due to unicode chars
//#[test]
//fn unicode_chars_false_positive(){
//    assert!(super::calc_content_spam_prob("😄Δπöä poggers;")<0.1);
//}
///// testing the probability calculation won't give a false negative due to unicode chars
//#[test]
//fn unicode_chars_false_negative(){
//    assert!(super::calc_content_spam_prob("😄Δπöä poggers; https://example.com @everyone")>0.9);
//}
