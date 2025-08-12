import React from 'react'

type Props = {}

const AccessDeniedPage = (props: Props) => {
  return (
    <div className="access-denied-page">
      <p style={{ textAlign: 'center' }}>You do not have permission to view this page.</p>
    </div>
  )
}

export default AccessDeniedPage