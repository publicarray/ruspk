import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatBoolean, formatArray, client_get_json } from '../../utils';

export async function getStaticProps(context) {
    const res = await fetch(`http://127.0.0.1:8080/api/build`)
    const data = await res.json()

    if (!data) {
        return {
            notFound: true,
        }
    }

    return {
        props: { data },
        revalidate: 5,
    }
}

export default function BuildPage({data}) {
    const url = `http://127.0.0.1:8080/api/build`
    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, package: "python38", version: "3.8.11", revision: "4", arch: "evansport", firmware: "7.0-41890", publisher: "hgy59", insert_date: "2021-08-31 22:47:07.280900", active: "true" },
    //     { id: 2, package: "python38", version: "3.8.11", revision: "4", arch: "armada38x, armadaxp, alpine4k, comcerto2k, monaco, alpine, armada375, armada370 ", firmware: "7.0-41890", publisher: "hgy59", insert_date: "2021-08-31 22:47:07.280900", active: "true" },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Package', accessor: 'package',},
        { Header: 'Upstream Version', accessor: 'upstream_version',},
        { Header: 'Revision', accessor: 'revision',},
        { Header: 'Architectures', accessor:  row => formatArray(row.architectures),},
        { Header: 'Firmware', accessor: 'firmware',},
        { Header: 'Publisher', accessor: 'publisher',},
        { Header: 'Insert Date', accessor: 'insert_date',},
        { Header: 'Active', accessor: row => formatBoolean(row.active) },
    ];


    // We'll start our table without any data
    // const [data, setData] = React.useState([])
    const [loading, setLoading] = React.useState(false)
    const [pageCount, setPageCount] = React.useState(0)
    const fetchData = false;
    // const fetchIdRef = React.useRef(0)

    // const fetchData = React.useCallback(({ pageSize, pageIndex }) => {
    // // function fetchData({ pageSize, pageIndex }) {
    //   // This will get called when the table needs new data
    //   // You could fetch your data from literally anywhere,
    //   // even a server. But for this example, we'll just fake it.

    //   // Give this fetch an ID
    //   const fetchId = ++fetchIdRef.current

    //   // Set the loading state
    //   setLoading(true)

    //     // Only update the data if this is the latest fetch
    //     if (fetchId === fetchIdRef.current) {
    //       const startRow = pageSize * pageIndex
    //       const endRow = startRow + pageSize
    //     //   const res = await fetch(`http://127.0.0.1:8080/api/build?index=${pageIndex}&size=${pageSize}`)
    //       const { data, error } = client_get_json(`http://127.0.0.1:8080/api/build?index=${pageIndex}&size=${pageSize}`) // client render
    //     //   const data = await res.json()
    //       setData(data)

    //       // Your server could send back total page count.
    //       // For now we'll just fake it, too
    //       setPageCount(Math.ceil(data.length / pageSize))

    //       setLoading(false)
    //     }
    // }, []);


    return (
        <Layout>
            <h1>Build</h1>
            <Table columns={columns} data={data} url={url} fetchData={fetchData} loading={loading} pageCount={pageCount}></Table>
            <Button>Add Build</Button>
        </Layout>
    );
}
