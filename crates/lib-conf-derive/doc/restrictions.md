## Restrictions

Restrictions on attribute usage are listed in a table under each description (if there are any).

The types of restrictions are:

**Type**               | **Description**
---------------------- | ---------------
**Mandatory**          | This attribute must be present
**Required attrs**     | Can only be used when the listed attributes are also present
**Incompatible attrs** | Cannot be used alongside any of the listed attributes
**Implied attrs**      | The listed attributes are (effectively) applied, and therefore can't be set
**Incompatible types** | Cannot be used on fields of the listed types
**Supported values**   | Supported values for key-value attributes (`#[config(attr = ..)]`)

If a restriction type is omitted from the respective attribute's table, it can be inferred as none.

Additional restrictions that don't fit into the above generic groups will be listed separately.
