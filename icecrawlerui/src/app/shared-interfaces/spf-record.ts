export interface SPFRecord
{
	domain: string,
	version: string,
	mechanisms: string[],
	qualifier: string,
	ip: string[],
	include: string[],
	all: string
}
