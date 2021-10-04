import Layout from "../../components/layout";
import Head from 'next/head'
import { useRouter } from 'next/router'
import useSWR from 'swr'
import { fetchJson, API, API_VER, CDN } from "../../utils";
import Link from 'next/link'
import React, {useState, useEffect} from 'react';

export default function PackageDetail(props) {
    const router = useRouter()
    const { name } = router.query;

    let [pkg, setPkg] = useState({})
    let [versions, setVersions] = useState({})
    let [isPkgLoading, setPkgLoading] = useState(true)
    let [isVerLoading, setVerLoading] = useState(true)
    let [latestRevision, setLatestRevision] = useState(1)

    React.useEffect(() => {
        if (!name) {
            return
        }
        fetch(`${API}/${API_VER}/package/${name}`).then(response => {
            response.json().then(r => {
                setPkg(r)
                setPkgLoading(false)
            })
        });
        fetch(`${API}/${API_VER}/version?q=${name}`).then(response => {
            response.json().then(r=>{
                setVersions(r)
                let latestRev = 1;
                r.forEach(ver => {
                    if (ver.revision > latestRev) {
                        latestRev = ver.revision
                    }
                });
                setLatestRevision(latestRev)
                setVerLoading(false)
            })
        })
    }, [name])
    // let pkg_resp = useSWR(`${API}/${API_VER}/package/${name}`, fetchJson);
    // let ver_resp = useSWR(`${API}/${API_VER}/version?q=${name}`, fetchJson);
    // let build_resp = useSWR(`${API}/${API_VER}/build/${name}`, fetchJson);

    if (isPkgLoading || isVerLoading) {
        return <p>Loading...</p>
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>
            <h1 className="text-4xl">{pkg.displayname}</h1>
            <img className="rounded-xl mb-2" src={`${CDN}/${pkg.name}/${latestRevision}/icon256.png`} />
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
