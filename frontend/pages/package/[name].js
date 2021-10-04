import Layout from "../../components/layout";
import Head from 'next/head'
import { useRouter } from 'next/router'
import useSWR from 'swr'
import { fetchJson, API, API_VER, CDN } from "../../utils";
import Link from 'next/link'
import React, {useState, useEffect} from 'react';
import Image from '../../components/image'

export default function PackageDetail(props) {
    const router = useRouter()
    const { name } = router.query;

    let [_package, setPackage] = useState({})
    let [versions, setVersions] = useState({})
    let [builds, setBuilds] = useState({})
    let [isPkgLoading, setPkgLoading] = useState(true)
    let [isVerLoading, setVerLoading] = useState(true)
    let [isBuildsLoading, setBuildsLoading] = useState(true)
    let [latestRevision, setLatestRevision] = useState(1)

    React.useEffect(() => {
        if (!name) {
            return
        }
        fetch(`${API}/${API_VER}/package/${name}`).then(response => {
            response.json().then(r => {
                setPackage(r)
                setPkgLoading(false)
            })
        });
        fetch(`${API}/${API_VER}/version?q=${name}`).then(response => {
            response.json().then(r => {
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
        fetch(`${API}/${API_VER}/build?q=${name}`).then(response => {
            response.json().then(r => {
                console.log(r);
                // https://gist.github.com/robmathers/1830ce09695f759bf2c4df15c29dd22d
                r = r.reduce(
                    (a, x) => {
                        let firmware = x['firmware'].split('-')[0] // strip build number (6.1-1234 -> 6.1)
                        let firmware_major = firmware.split('.')[0] + ".x" // format as 6.x
                        if (firmware < 3) { // needs to be updated when new versions are released
                            firmware_major = "SRM " + firmware_major + ":" // SRM 1.x:
                        } else {
                            firmware_major = "DSM " + firmware_major + ":" // DSM 6.x:
                        }
                        a[firmware_major] = a[firmware_major] || [];
                        a[firmware_major].push(x);
                        return a;
                    },
                    []
                )
                setBuilds(r)
                setBuildsLoading(false)
            })
        })
    }, [name])
    // let pkg_resp = useSWR(`${API}/${API_VER}/package/${name}`, fetchJson);
    // let ver_resp = useSWR(`${API}/${API_VER}/version?q=${name}`, fetchJson);
    // let build_resp = useSWR(`${API}/${API_VER}/build/${name}`, fetchJson);

    if (isPkgLoading || isVerLoading || isBuildsLoading) {
        return <p>Loading...</p>
    }

    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>
            <div className="flex">
                <Image className="rounded-xl mb-2" src={`${CDN}/${_package.name}/${latestRevision}/icon256.png`} width="256" height="256" alt="" />
                <div className="my-4">
                    <h1 className="mb-2 text-4xl">{_package.displayname}</h1>
                    <p>Author: {_package.author}</p>
                    <p dangerouslySetInnerHTML={{ __html: _package.description }}></p>
                    {/* <p>Version: {pkg.version}-{pkg.revision}</p> */}
                    {/* <p>Date: {pkg.insert_date}</p> */}
                </div>
            </div>
            <div>
                {versions.map(version => { return (<div key={version.id}>
                    <hr></hr>
                    <div className="my-2">
                        <h3 className="font-bold text-xl">Version <span className="font-medium">{version.upstream_version}-{version.revision}</span></h3>
                        <p className="mb-2" dangerouslySetInnerHTML={{ __html: version.changelog }}></p>
                        <h3 className="font-bold text-xl">Date</h3>
                        <p className="mb-2">{version.insert_date}</p>
                        <h3 className="font-bold text-xl">Downloads / Architectures / Builds</h3>
                        {Object.entries(builds).map(([k,v],i) => { return (
                            <div key={i}>
                                <p>{k}</p>
                                <p className="flex flex-wrap">{v.map(build => { return (
                                    <>
                                        {build.revision == version.revision &&
                                            <>{build.architectures.map( arch =>
                                                <a key={`${build.id}-${arch}`}
                                                    className="px-3 py-1 m-1 bg-gray-800 text-white dark:text-gray-100 dark:bg-black rounded-full hover:bg-gray-700 dark:hover:bg-gray-700 dark:hover:text-white"
                                                    href={`${CDN}/${_package.name}/${version.revision}/${_package.name}.v${version.revision}.f${build.firmware.split('-')[1]}[${build.architectures.join('-')}].spk`}>{arch}</a>
                                            )}</>
                                        }
                                    </>
                                )})}
                                </p>
                            </div>
                        )})}
                    </div>
                </div>)})}
            </div>
            <br></br>
        </Layout>
  )
}
