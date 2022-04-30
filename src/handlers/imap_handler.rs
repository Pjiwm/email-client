use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;
pub fn search(cn: &str) -> Result<Vec<String>> {
    let mut results = Vec::<String>::new();
    let mut ldap = LdapConn::new("ldap://ldap.itd.umich.edu")?;
    let (rs, _res) = ldap.search(
        "dc=umich,dc=edu",
        Scope::Subtree,
        format!("(cn={})", cn).as_str(),
        vec![""]
    )?.success()?;
    println!("Ldap res: {}", _res);
    println!("Entries ({}):", rs.len());
    for entry in rs {
        results.push(format!("{:?}", SearchEntry::construct(entry)));
    }
    Ok(results)
}