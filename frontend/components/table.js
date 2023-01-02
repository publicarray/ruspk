import {
    createColumnHelper,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table";
import { fetchJson, fetchJsonWithAuth } from "../utils";
import useSWR from "swr";
import React from "react";
import { useState } from "react";

// https://react-table.tanstack.com/
// https://react-table-v7.tanstack.com/docs/quick-start
export default function Table(props) {
    let pageIndex = props.pageIndex;
    if (!pageIndex || pageIndex <= 0) {
        pageIndex = 1
        // props.setPageIndex(1);
    }
    let pageSize = 15;
    let sortBy = "";
    let filters = "";
    let r = useSWR(`${props.url}?page=${pageIndex}&size=${pageSize}`, fetchJsonWithAuth);
    let error = r.error;
    let new_data = r.data;
    React.useEffect(() => {
        if (!new_data) {
            props.setData([])
        } else {
            props.setData(new_data)
        }
    }, [new_data, props])
        // if (!props.setData) {
        //     return <></>;
        // }
    if (error) {console.error(error)};
    // if (!new_data) return <div>loading...</div>;
    // setData(r.data)
    // return <div>loading...</div>;

    const table = useReactTable({
        data: props.data,
        columns: props.columns,
        getCoreRowModel: getCoreRowModel(),
    });

    return (
        <div className="flex overflow-x-auto">
            <style jsx>{`
                td[data-header="architectures"] {
                    white-space: normal;
                }
            `}</style>
            <div className="w-full">
                <div className="bg-white dark:bg-gray-900 shadow-md rounded my-6">
                    <table className="w-full table-auto">
                        <thead>
                            {table.getHeaderGroups().map(headerGroup => (
                                <tr
                                    className="bg-gray-200 text-black dark:bg-black dark:text-gray-300 uppercase text-sm leading-normal"
                                    key={headerGroup.id}
                                >
                                    {headerGroup.headers.map((header) => (
                                        <th
                                            className="py-3 px-6 text-left"
                                            key={header.id}
                                        >{header.isPlaceholder
                                            ? null :
                                            flexRender(
                                                header.column.columnDef.header,
                                                header.getContext()
                                            )}
                                        </th>
                                    ))}
                                </tr>
                            ))}
                        </thead>
                        <tbody className="text-gray-700 dark:text-gray-300 text-sm">
                            {table.getRowModel().rows.map((row) => (
                                <tr
                                    className="border-b border-gray-200 hover:bg-gray-100 dark:border-gray-600 dark:hover:bg-gray-800"
                                    key={row.id}
                                >
                                    {row.getVisibleCells().map((cell) => (
                                        <td
                                            className="py-3 px-6 text-left"
                                            // className="py-3 px-6 text-left sm:whitespace-nowrap"
                                            key={cell.id}
                                        >
                                            {flexRender(
                                                cell.column.columnDef.cell,
                                                cell.getContext()
                                            )}
                                        </td>
                                    ))}
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
}
