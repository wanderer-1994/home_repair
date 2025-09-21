let SameSite = < none | lax | strict >

let CookieConfig = { useHttps : Bool, sameSite : SameSite, cookieDomain : Text }

in  { CookieConfig, SameSite }
