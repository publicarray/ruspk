// import React from 'react'
import { useTable } from 'react-table'
import DetailBtn from './detail-btn'
import EditBtn from './edit-btn'
import DeleteBtn from './delete-btn'
import { fetch_json } from "../utils";
import useSWR from 'swr'
import ReactPaginate from 'react-paginate';
import React, { useState, useEffect } from 'react';
import { useRouter } from "next/router"

// https://react-table.tanstack.com/
export default function Table({columns, data, url, loading, currentPage, pageCount}) {
    const [data1, setData] = useState([])
    const router = useRouter()

    useEffect(() => {
        if (data) {
          if (data.error) {
            return
          } else {
            setData(data)
          }
        }
      }, [data])

    const paginationHandler = (page) => {
        router.query.page = page.selected + 1
        router.push({
            pathname: router.pathname,
            query: router.query,
        });
    };

    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
    } = useTable({
            columns,
            data:data1,
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


                <ReactPaginate
                    previousLabel={'previous'}
                    nextLabel={'next'}
                    breakLabel={''}
                    containerClassName={'pagination'}
                    pageClassName={'inline-block'}
                    pageLinkClassName={'py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700'}
                    previousClassName={'inline-block'}
                    previousLinkClassName={'py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700'}
                    nextClassName={'inline-block'}
                    nextLinkClassName={'py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700'}
                    breakClassName={'inline-block'}
                    breakLinkClassName={'py-1 px-3 text-white rounded-lg bg-blue-500 hover:bg-blue-700'}
                    activeClassName={'inline-block'}
                    activeLinkClassName={'bg-blue-700'}
                    initialPage={currentPage-1}
                    pageCount={pageCount}
                    marginPagesDisplayed={0}
                    pageRangeDisplayed={0}
                    onPageChange={paginationHandler}
                />

                </div>
            </div>
        </div>
    )
}
