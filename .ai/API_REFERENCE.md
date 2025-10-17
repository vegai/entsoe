# ENTSO-E API Reference

**Last Updated:** 2025-10-11

**Note:** This document captures the ENTSO-E API as understood at the time of writing. Always verify current behavior with the official documentation as the API may change.

## Overview

This document provides detailed information about the ENTSO-E Transparency Platform API, including endpoints, parameters, response formats, and common patterns.

**Official Documentation:** https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html

## Base URL

```
https://web-api.transparency.entsoe.eu/api
```

All requests are made via HTTP GET with query parameters.

## Authentication

Authentication is done via a security token passed as a query parameter.

**Parameter:** `securityToken`

### Getting a Token

1. Register at https://transparency.entsoe.eu/
2. Navigate to "My Account Settings"
3. Generate a new Web API Security Token
4. Store it securely (treat it like a password)

**Example:**
```
?securityToken=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

## Common Parameters

### Document Types

Document types identify what kind of data you're requesting:

| Code | Description |
|------|-------------|
| A44  | Price Document (Day-ahead prices) |
| A65  | System total load |
| A68  | Installed generation capacity per type |
| A69  | Wind and solar forecast |
| A70  | Load forecast margin |
| A71  | Generation forecast |
| A72  | Reservoir filling information |
| A73  | Actual generation |
| A74  | Wind and solar generation |
| A75  | Actual generation per type |

### Time Parameters

Times must be in UTC and formatted as: **YYYYMMDDhhmm**

- `periodStart` - Start of the period (inclusive)
- `periodEnd` - End of the period (exclusive)

**Examples:**
- `20240115000000` - January 15, 2024 at 00:00 UTC
- `20240115120000` - January 15, 2024 at 12:00 UTC

**Note:** Always use UTC, regardless of the bidding zone's local timezone.

### Area Parameters

- `in_Domain` - The area where energy is consumed/delivered to
- `out_Domain` - The area where energy is produced/delivered from

For prices, typically use `in_Domain` with the bidding zone code.

### Process Types

| Code | Description |
|------|-------------|
| A01  | Day ahead |
| A02  | Intra day incremental |
| A16  | Realised |
| A18  | Intraday total |
| A31  | Week ahead |
| A32  | Month ahead |
| A33  | Year ahead |
| A40  | Forecast |

## Day-Ahead Prices

### Endpoint

```
GET https://web-api.transparency.entsoe.eu/api
```

### Required Parameters

| Parameter | Value | Description |
|-----------|-------|-------------|
| documentType | A44 | Price document |
| in_Domain | EIC Code | Bidding zone (see table below) |
| out_Domain | EIC Code | Same as in_Domain for prices |
| periodStart | YYYYMMDDhhmm | Start time (UTC) |
| periodEnd | YYYYMMDDhhmm | End time (UTC) |
| securityToken | Your token | Authentication |

### Example Request

```
https://web-api.transparency.entsoe.eu/api?
  documentType=A44&
  in_Domain=10YDE-VE-------2&
  out_Domain=10YDE-VE-------2&
  periodStart=202401150000&
  periodEnd=202401160000&
  securityToken=YOUR_TOKEN
```

### Response Format

XML document following IEC 62325-451 standard:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Publication_MarketDocument xmlns="urn:iec62325.351:tc57wg16:451-3:publicationdocument:7:0">
    <mRID>...</mRID>
    <revisionNumber>1</revisionNumber>
    <type>A44</type>
    <sender_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</sender_MarketParticipant.mRID>
    <sender_MarketParticipant.marketRole.type>A32</sender_MarketParticipant.marketRole.type>
    <receiver_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</receiver_MarketParticipant.mRID>
    <receiver_MarketParticipant.marketRole.type>A33</receiver_MarketParticipant.marketRole.type>
    <createdDateTime>2024-01-14T13:00:00Z</createdDateTime>
    <period.timeInterval>
        <start>2024-01-15T00:00Z</start>
        <end>2024-01-16T00:00Z</end>
    </period.timeInterval>
    <TimeSeries>
        <mRID>1</mRID>
        <businessType>A62</businessType>
        <in_Domain.mRID codingScheme="A01">10YDE-VE-------2</in_Domain.mRID>
        <out_Domain.mRID codingScheme="A01">10YDE-VE-------2</out_Domain.mRID>
        <currency_Unit.name>EUR</currency_Unit.name>
        <price_Measure_Unit.name>MWH</price_Measure_Unit.name>
        <curveType>A01</curveType>
        <Period>
            <timeInterval>
                <start>2024-01-15T00:00Z</start>
                <end>2024-01-16T00:00Z</end>
            </timeInterval>
            <resolution>PT60M</resolution>
            <Point>
                <position>1</position>
                <price.amount>45.67</price.amount>
            </Point>
            <Point>
                <position>2</position>
                <price.amount>43.21</price.amount>
            </Point>
            <!-- ... more points ... -->
        </Period>
    </TimeSeries>
</Publication_MarketDocument>
```

### Response Fields

- `mRID` - Market document ID
- `type` - Document type (A44 for prices)
- `createdDateTime` - When the document was created
- `TimeSeries` - Contains the actual data
  - `currency_Unit.name` - Currency (always EUR for day-ahead prices, regardless of zone)
  - `price_Measure_Unit.name` - Unit (typically MWH)
  - `Period` - Time period with resolution
    - `resolution` - Time resolution (PT60M = 60 minutes, PT15M = 15 minutes)
    - `Point` - Individual price points
      - `position` - Index (1-based, corresponds to time slots)
      - `price.amount` - Price value

### Resolution Codes

- `PT15M` - 15-minute intervals (96 points per day)
- `PT60M` - 60-minute intervals (24 points per day)

## Bidding Zones (EIC Codes)

European electricity markets are divided into bidding zones. Each has a unique EIC (Energy Identification Code).

### Major Bidding Zones

| Country/Zone | Code | EIC Code |
|--------------|------|----------|
| Germany-Luxembourg | DE-LU | 10Y1001A1001A82H |
| Germany | DE | 10YDE-VE-------2 |
| Austria | AT | 10YAT-APG------L |
| Belgium | BE | 10YBE----------2 |
| Denmark (DK1) | DK1 | 10YDK-1--------W |
| Denmark (DK2) | DK2 | 10YDK-2--------M |
| Finland | FI | 10YFI-1--------U |
| France | FR | 10YFR-RTE------C |
| Italy North | IT-North | 10Y1001A1001A73I |
| Netherlands | NL | 10YNL----------L |
| Norway (NO1) | NO1 | 10YNO-1--------2 |
| Norway (NO2) | NO2 | 10YNO-2--------T |
| Norway (NO3) | NO3 | 10YNO-3--------J |
| Norway (NO4) | NO4 | 10YNO-4--------9 |
| Norway (NO5) | NO5 | 10Y1001A1001A48H |
| Poland | PL | 10YPL-AREA-----S |
| Spain | ES | 10YES-REE------0 |
| Sweden (SE1) | SE1 | 10Y1001A1001A44P |
| Sweden (SE2) | SE2 | 10Y1001A1001A45N |
| Sweden (SE3) | SE3 | 10Y1001A1001A46L |
| Sweden (SE4) | SE4 | 10Y1001A1001A47J |
| Switzerland | CH | 10YCH-SWISSGRIDZ |
| United Kingdom | GB | 10YGB----------A |

**Note:** Some countries are divided into multiple bidding zones due to transmission constraints.

### Finding EIC Codes

Complete list available at: https://www.entsoe.eu/data/energy-identification-codes-eic/

## Error Responses

### Error XML Format

When an error occurs, the API returns an XML error document:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Acknowledgement_MarketDocument xmlns="urn:iec62325.351:tc57wg16:451-3:acknowledgementdocument:7:0">
    <mRID>...</mRID>
    <createdDateTime>2024-01-15T10:30:00Z</createdDateTime>
    <sender_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</sender_MarketParticipant.mRID>
    <sender_MarketParticipant.marketRole.type>A32</sender_MarketParticipant.marketRole.type>
    <receiver_MarketParticipant.mRID codingScheme="A01">10X1001A1001A450</receiver_MarketParticipant.mRID>
    <receiver_MarketParticipant.marketRole.type>A39</receiver_MarketParticipant.marketRole.type>
    <Reason>
        <code>999</code>
        <text>No matching data found</text>
    </Reason>
</Acknowledgement_MarketDocument>
```

### Common Error Codes

| Code | Description | Solution |
|------|-------------|----------|
| 401 | Unauthorized | Check your security token |
| 400 | Bad Request | Verify all required parameters |
| 999 | No matching data found | Check bidding zone, time period, and data availability |
| 429 | Too Many Requests | Implement rate limiting |

### HTTP Status Codes

- `200 OK` - Success (even for "no data" errors in XML)
- `401 Unauthorized` - Invalid or missing token
- `400 Bad Request` - Missing required parameters
- `429 Too Many Requests` - Rate limit exceeded

## Rate Limits

**Official Limit:** Not explicitly documented, but best practices:
- Max ~400 requests per minute
- Implement exponential backoff for retries
- Cache responses when appropriate
- Don't hammer the API with identical requests

## Best Practices

### 1. Time Ranges

- Don't request more than 1 year at a time
- For day-ahead prices, request 1-7 days typically
- Remember: `periodEnd` is exclusive

### 2. Error Handling

- Always check for `Acknowledgement_MarketDocument` (error responses)
- Handle "No matching data found" gracefully (it's common)
- Implement retry logic with exponential backoff

### 3. Caching

- Day-ahead prices for past days don't change - cache them
- Current day prices may be updated - refresh periodically
- Future prices are published ~13:00 CET the day before

### 4. Time Zones

- **Always use UTC** for API requests
- Convert to local time zones only for display
- Be aware of DST changes in European zones

### 5. Data Availability

- Not all zones publish all data types
- Historical data may have gaps
- Some data is published with delays

## Example Use Cases

### Fetch Next 24 Hours of Prices

```
# Today at 00:00 UTC
periodStart=202401150000

# Tomorrow at 00:00 UTC (24 hours later)
periodEnd=202401160000

# This gives you 24 hourly prices (positions 1-24)
```

### Fetch Prices for a Specific Hour

```
# 14:00-15:00 on January 15, 2024
periodStart=202401151400
periodEnd=202401151500

# This gives you 1 price point
```

### Fetch a Week of Prices

```
# January 15-22, 2024
periodStart=202401150000
periodEnd=202401220000

# This gives you 168 hourly prices (24 * 7)
```

## Parsing Tips

### Position to Timestamp Conversion

The `position` field is 1-based and corresponds to time slots:

```
Position 1 = periodStart
Position 2 = periodStart + resolution
Position 3 = periodStart + (2 * resolution)
...
```

For hourly data (PT60M):
- Position 1 = 00:00-01:00
- Position 2 = 01:00-02:00
- Position 24 = 23:00-00:00

### Multiple TimeSeries

Some responses may contain multiple `TimeSeries` elements. This happens when:
- Data comes from different sources
- Different market participants publish data
- Data spans across different periods

Handle this by parsing all `TimeSeries` elements and merging/sorting the results.

## Testing and Development

### Test Parameters

For testing, use well-known zones with reliable data:

**Germany (DE):**
```
in_Domain=10Y1001A1001A82H
out_Domain=10Y1001A1001A82H
```

**France (FR):**
```
in_Domain=10YFR-RTE------C
out_Domain=10YFR-RTE------C
```

### Sandbox Environment

⚠️ ENTSO-E does not provide a sandbox environment. All requests hit the production API.

### Storing Test Fixtures

Save real API responses as fixtures for testing:
```bash
curl "https://web-api.transparency.entsoe.eu/api?..." > tests/fixtures/response.xml
```

## Additional Resources

- [ENTSO-E Transparency Platform](https://transparency.entsoe.eu/)
- [RESTful API Guide (PDF)](https://transparency.entsoe.eu/content/static_content/Static%20content/web%20api/Guide.html)
- [EIC Codes Registry](https://www.entsoe.eu/data/energy-identification-codes-eic/)
- [IEC 62325 Standard](https://www.iec.ch/) - XML schema standard used by ENTSO-E