import React from 'react'
import Layout from "../../components/layout";
import Button from "../../components/button";
import DeleteBtn from "../../components/delete-btn";
import TablePaginate from "../../components/table-paginate";
import { Dialog } from "@headlessui/react";
import { formatBoolean, formatArray } from '../../utils';
import { useRouter } from 'next/router'

export default function BuildPage() {
    const router = useRouter()
    const url = `http://127.0.0.1:8080/api/build`
    // let edit = async function (row) {
    //     console.log("edit", row)
    //     const response = await fetch(`${url}/${row.values.id}`, {
    //         method: "PUT",
    //         headers: {
    //           "Content-type": "application/json; charset=UTF-8",
    //         },
    //         body: JSON.stringify(row.values),
    //     });
    //     let data = await response.json()
    //     console.log("delete-return", data)
    // }

    let del = async function (row, data) {
        console.log("deleting", row)
        const response = await fetch(`${url}/${row.values.id}`, {
            method: "DELETE",
        });
        console.log("response", response)
        if (response.ok) {
            let response_data = await response.json()
            console.log("response", response_data)
            data.splice(row.index, 1) // update table
            router.push("/admin/build", undefined, {shallow: true}) // force refresh of internal data
        }
    }

    let columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'Package', accessor: 'package',},
        { Header: 'Upstream Version', accessor: 'upstream_version',},
        { Header: 'Revision', accessor: 'revision',},
        { Header: 'Architectures', accessor: row => formatArray(row.architectures),},
        { Header: 'Firmware', accessor: 'firmware',},
        { Header: 'Publisher', accessor: 'publisher',},
        { Header: 'Insert Date', accessor: 'insert_date',},
        { Header: 'Active', accessor: 'active', Cell: ({ value }) => formatBoolean(value) },
        {
            Header: "Actions",
            accessor: "actions",
            Cell: (props) => {
                const row = props.row;
                return (
                    <div>
                        {/* <span onClick={() => edit(row)}>
                            <i className="far fa-edit action mr-2">Edit</i>
                        </span> */}
                        <span onClick={i => del(row, props.data)}>
                            <DeleteBtn></DeleteBtn>
                        </span>
                    </div>
                );
            },
        }
    ];

    return (
        <Layout>
            <h1>Build</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button>Add Build</Button>
        </Layout>
    );
}
