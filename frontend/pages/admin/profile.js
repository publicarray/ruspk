// import { withIronSession } from "next-iron-session";
import Layout from "../../components/layout-admin";
import Button from "../../components/button";

function handler(req, res, session) {
    const user = req.session.get("user");
    res.send({ user });
}

//   export default withIronSession(handler, {
//     password: "complex_password_at_least_32_characters_long",
//     cookieName: "myapp_cookiename",
//     // if your localhost is served on http:// then disable the secure flag
//     cookieOptions: {
//       secure: process.env.NODE_ENV === "production",
//     },
//   });


export default function IndexPage() {




  return (
      <Layout>
        <h1>Profile</h1>
        <h2></h2>
        <h2>Permissions</h2>
        <ul>Package Administrator</ul>
        <ul>Developer</ul>
        <ul>Administrator</ul>
        <h2>API Key</h2>
        <p>You have permission to push packages to the repository. Use the following API key to publish packages:</p>
        <pre><code></code></pre>
        <Button>Generate API Key</Button>
      </Layout>
  )
}
