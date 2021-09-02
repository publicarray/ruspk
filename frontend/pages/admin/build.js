import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";

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
    }
}

export default function BuildPage({data}) {
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
        { Header: 'Upstream Version', accessor: 'version',},
        { Header: 'Revision', accessor: 'revision',},
        { Header: 'Architectures', accessor: 'arch',},
        { Header: 'Firmware', accessor: 'firmware',},
        { Header: 'Publisher', accessor: 'publisher',},
        { Header: 'Insert Date', accessor: 'insert_date',},
        { Header: 'Active', accessor: 'active',},
    ];

    return (
        <Layout>
            <h1>Build</h1>
            <Table columns={columns} data={data}></Table>
            <Button>Add Build</Button>
        </Layout>
    );
}
