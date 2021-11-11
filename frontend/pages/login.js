import Layout from "../components/layout";
import Login from "../components/login";
import Head from 'next/head';
import {postJsonForm, API} from '../utils';

export default function LoginPage() {
    let onSubmit = async function (event) {
        const response = await postJsonForm(`${API}/login`, event, []);
        // const response = await postJsonForm(`/api/login`, event, []);
        console.log(response);
        localStorage.setItem('jwt', response);
        //Router.push('/admin');
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity - Login</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300">
                <Login onSubmit={onSubmit}></Login>
            </main>
        </Layout>
    )
}
