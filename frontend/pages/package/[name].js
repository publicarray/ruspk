import Layout from "../../components/layout";
import Head from 'next/head'
import { useRouter } from 'next/router'
import useSWR from 'swr'
import { fetchJson } from "../../utils";
import Link from 'next/link'
import React, {useState, useEffect} from 'react';

export default function PackageDetail(props) {
    const router = useRouter()
    const { name } = router.query;

    let [pkg, setPkg] = useState({})
    let [versions, setVersions] = useState({})
    let [isPkgLoading, setPkgLoading] = useState(true)
    let [isVerLoading, setVerLoading] = useState(true)
    React.useEffect(() => {
        if (!name) {
            return
        }
        fetch(`http://127.0.0.1:8080/api/package/${name}`).then(response => {
            response.json().then(r => {
                setPkg(r)
                setPkgLoading(false)
            })
        });
        fetch(`http://127.0.0.1:8080/api/version?q=${name}`).then(response => {
            response.json().then(r=>{
                setVersions(r)
                setVerLoading(false)
            })
        })
    }, [name])
    // let pkg_resp = useSWR(`http://127.0.0.1:8080/api/package/${name}`, fetchJson);
    // let ver_resp = useSWR(`http://127.0.0.1:8080/api/version?q=${name}`, fetchJson);
    // let build_resp = useSWR(`http://127.0.0.1:8080/api/build/${name}`, fetchJson);


    if (isPkgLoading || isVerLoading) {
        return <p>Loading...</p>
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>
            <h1>Detail Page: {pkg.displayname}</h1>
            <p>Author: {pkg.author}</p>
            <p>Description: {pkg.description}</p>
            <p>Version: {pkg.version}-{pkg.revision}</p>
            <p>Date: {pkg.insert_date}</p>
            <br></br>
            <div>
                {versions.map(row => { return (<div key={row.id}>
                    <p>Version: {row.upstream_version}-{row.revision}</p>
                    <p>Date: {row.insert_date}</p>
                </div>)})}
            </div>
            <br></br>
        </Layout>
  )
}
