import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";

export async function getStaticProps(context) {
    const res = await fetch(`http://127.0.0.1:8080/api/package`)
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

export default function PackagePage({data}) {
    let [isOpen, setIsOpen] = useState(true);

    function closeModal() {
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, name: "bitlbee", author: "Diaoul", maintainers: "", insert_date: "2015-01-28 22:00:44.967691" },
    // ];

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Name', accessor: 'name',},
        { Header: 'Author', accessor: 'author',}, // author_user_id
        // { Header: 'Maintainers', accessor: 'maintainers',},
        { Header: 'Insert Date', accessor: 'insert_date',},
    ];

    return (
        <Layout>
            <h1>Package</h1>
            <Table columns={columns} data={data}></Table>
            <Button>Add Package</Button>
            <Button>Edit Package</Button>
        </Layout>
    );
}
