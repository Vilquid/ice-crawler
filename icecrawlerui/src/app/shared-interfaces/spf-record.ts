export interface SPFRecord
{
	version: string,
	mechanisms: string[],
	qualifier: string,
	ip: string[],
	include: string[],
	all: string,
	note: string
}
