import React from 'react'
import Layout from "../../components/layout";
import Button from "../../components/button";
import Table from "../../components/table";
import Model from "../../components/model";
import { Dialog } from "@headlessui/react";
import { formatBoolean, formatArray, client_get_json } from '../../utils';


export async function getServerSideProps({ query }) {
    const url = `http://127.0.0.1:8080/api/build`
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
            pageCount: 100
        }
    }
}

export default function BuildPage({data, currentPage, pageCount}) {
    const url = `http://127.0.0.1:8080/api/build`

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Package', accessor: 'package',},
        { Header: 'Upstream Version', accessor: 'upstream_version',},
        { Header: 'Revision', accessor: 'revision',},
        { Header: 'Architectures', accessor: row => formatArray(row.architectures),},
        { Header: 'Firmware', accessor: 'firmware',},
        { Header: 'Publisher', accessor: 'publisher',},
        { Header: 'Insert Date', accessor: 'insert_date',},
        { Header: 'Active', accessor: 'active', Cell: ({ value }) => formatBoolean(value) },
    ];

    return (
        <Layout>
            <h1>Build</h1>
            <Table columns={columns} data={data} currentPage={currentPage} pageCount={pageCount}></Table>
            <Button>Add Build</Button>
        </Layout>
    );
}
