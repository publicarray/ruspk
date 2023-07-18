import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import useSWR from 'swr'
import { fetchJsonWithAuth, API, API_VER } from "../../utils";

export const runtime = 'edge';

export default function ProfilePage() {
    const url = `${API}/${API_VER}/profile`;
    let { data, error, isLoading } = useSWR(`${url}`, fetchJsonWithAuth);
    if (error) {console.error(error); return<div>failed to load</div>};
    if (isLoading) return <div>loading...</div>;
    if (!data) return <div>loading...</div>;

    const listPermissions = data.roles.map(role =>
        <li key={role.id.toString()}>{role.description}</li>
    )

    return (
      <Layout>
        <h1 className="font-semibold text-xl">{data.username}&apos;s Profile</h1>
        <h2 className="font-semibold text-l">Permissions</h2>
        <ul>{listPermissions}</ul>
        <h2 className="font-semibold text-l">API Key</h2>
        <p>You have permission to push packages to the repository. Use the following API key to publish packages:</p>
        <pre><code className="">{data.api_key}</code></pre>
        <Button>Generate API Key</Button>
      </Layout>
  )
}
