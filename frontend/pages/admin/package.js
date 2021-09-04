import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";

export async function getServerSideProps({ query }) {
    const url = `http://127.0.0.1:8080/api/package`
    const page = query.page || 1; //if page empty we request the first page
    const res = await fetch(`${url}?page=${page}&size=15`)
    const data = await res.json()

    if (!data) {
        return {
            notFound: true,
        }
    }

    return {
        props: {
            data,
            currentPage: page,
            pageCount: page+1
        }
    }
}

export default function PackagePage({data, currentPage, pageCount}) {
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
            <Table columns={columns} data={data} currentPage={currentPage} pageCount={pageCount}></Table>
            <Button>Add Package</Button>
            <Button>Edit Package</Button>
        </Layout>
    );
}
