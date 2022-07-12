<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET - Albums All</name>
   <tag></tag>
   <elementGuidId>39fa86b4-63db-4bd9-886f-b73f77a2d413</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/albums</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, '[0].userId', '1')
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].title', 'quidem molestiae enim')


WS.verifyElementPropertyValue(response, '[1].userId', '1')
WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].title', 'sunt qui excepturi placeat culpa')


WS.verifyElementPropertyValue(response, '[2].userId', '1')
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].title', 'omnis laborum odio')


WS.verifyElementPropertyValue(response, '[3].userId', '1')
WS.verifyElementPropertyValue(response, '[3].id', '4')
WS.verifyElementPropertyValue(response, '[3].title', 'non esse culpa molestiae omnis sed optio')


WS.verifyElementPropertyValue(response, '[4].userId', '1')
WS.verifyElementPropertyValue(response, '[4].id', '5')
WS.verifyElementPropertyValue(response, '[4].title', 'eaque aut omnis a')


WS.verifyElementPropertyValue(response, '[5].userId', '1')
WS.verifyElementPropertyValue(response, '[5].id', '6')
WS.verifyElementPropertyValue(response, '[5].title', 'natus impedit quibusdam illo est')


WS.verifyElementPropertyValue(response, '[6].userId', '1')
WS.verifyElementPropertyValue(response, '[6].id', '7')
WS.verifyElementPropertyValue(response, '[6].title', 'quibusdam autem aliquid et et quia')


WS.verifyElementPropertyValue(response, '[7].userId', '1')
WS.verifyElementPropertyValue(response, '[7].id', '8')
WS.verifyElementPropertyValue(response, '[7].title', 'qui fuga est a eum')


WS.verifyElementPropertyValue(response, '[8].userId', '1')
WS.verifyElementPropertyValue(response, '[8].id', '9')
WS.verifyElementPropertyValue(response, '[8].title', 'saepe unde necessitatibus rem')


WS.verifyElementPropertyValue(response, '[9].userId', '1')
WS.verifyElementPropertyValue(response, '[9].id', '10')
WS.verifyElementPropertyValue(response, '[9].title', 'distinctio laborum qui')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
