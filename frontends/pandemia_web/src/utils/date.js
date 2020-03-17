import {format, setMonth} from 'date-fns' 
import id from 'date-fns/locale/id'

export const date_format_1 = "DD MMMM YYYY"
export const date_format_2 = "DD-MM-YY HH:mm"

export const dateFormat = (dtStr, dtFormat = date_format_1) => {
  return format(dtStr, dtFormat, {locale: id})
}

export const getMonthName = (monthNum) => {
  return format(setMonth(new Date(), monthNum), 'MMMM')
}

