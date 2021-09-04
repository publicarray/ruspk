import { useTable } from 'react-table'
import { fetch_json } from "../utils";
import useSWR from 'swr'
import React from 'react';

// https://react-table.tanstack.com/
export default function Table({pageIndex, columns, url}) {
    if (!pageIndex || pageIndex <= 0) {
        pageIndex = 1
    }
    console.log(`${url}?page=${pageIndex}&size=10`);
    let { data, error } = useSWR(`${url}?page=${pageIndex}&size=10`, fetch_json);
    if (error) {
        console.error(error)
        return (<div>failed to load</div>)
    }
    if (!data) {
        data = []
    }

    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
    } = useTable({
            columns,
            data,
        },
    )

    return (
        <div className="flex overflow-x-auto">
            <div className="w-full">
                <div className="bg-white shadow-md rounded my-6">
                    <table className="w-full table-auto" {...getTableProps()}>
                        <thead>
                            {
                                headerGroups.map(headerGroup => (
                                    <tr className="bg-gray-200 text-black uppercase text-sm leading-normal" {...headerGroup.getHeaderGroupProps()}>
                                        {
                                            headerGroup.headers.map(column => (
                                                <th className="py-3 px-6 text-left" {...column.getHeaderProps()}>{column.render('Header')}</th>
                                            ))
                                        }
                                    </tr>
                                ))
                            }
                        </thead>
                        <tbody className="text-gray-700 text-sm" {...getTableBodyProps()}>
                            {rows.map(row => {
                                prepareRow(row)
                                return (
                                    <tr className="border-b border-gray-200 hover:bg-gray-100" {...row.getRowProps()}>
                                        {row.cells.map(cell => {
                                            return (
                                                <td className="py-3 px-6 text-left" {...cell.getCellProps()}>
                                                    {cell.render('Cell')}
                                                </td>
                                            )
                                        })}
                                    </tr>
                                )
                            })}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    )
}
