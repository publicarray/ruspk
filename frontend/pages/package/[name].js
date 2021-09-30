import Layout from "../../components/layout";
import Head from 'next/head'
import { useRouter } from 'next/router'

export default function PackageDetail(props) {

    const router = useRouter()
    const { name } = router.query

    const url = `http://127.0.0.1:8080/api/package/${name}`


    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>
            <h1>Detail Page: {name}</h1>
        </Layout>
  )
}
