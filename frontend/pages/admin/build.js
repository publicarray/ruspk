import React from 'react'
import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { Dialog } from "@headlessui/react";
import { formatBoolean, formatArray } from '../../utils';

export default function BuildPage({data}) {
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
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button>Add Build</Button>
        </Layout>
    );
}
