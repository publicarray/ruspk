import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";

export async function getStaticProps(context) {
    const res = await fetch(`http://127.0.0.1:8080/api/screenshot`)
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

export default function ScreenshotPage({data}) {
    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, package: "Syncthing", image: "https://packages.synocommunity.com/syncthing/screenshot_1.jpg" },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Package', accessor: 'package',},
        { Header: 'Image', accessor: 'path',},
    ];

    return (
        <Layout>
            <h1>Screenshot</h1>
            <Table columns={columns} data={data}></Table>
            <Button>Add Firmware</Button>
        </Layout>
    );
}
